use std::borrow::Cow;
use std::future::Future;
use std::sync::Arc;

use rmcp::handler::server::ServerHandler;
use rmcp::model::*;
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServiceExt};
use crate::tools::CodegenomeTools;

/// Start the MCP server on stdio.
pub async fn run_stdio(store_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let handler = CodegenomeTools::new(store_dir);
    let transport = rmcp::transport::stdio();
    let service = handler.serve(transport).await?;
    service.waiting().await?;
    Ok(())
}

impl ServerHandler for CodegenomeTools {
    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, McpError>> + Send + '_ {
        let tools = vec![
            make_tool("codegenome_context", "Retrieve context around a file:line"),
            make_tool("codegenome_impact", "Blast radius from a file:line change"),
            make_tool("codegenome_detect_changes", "Map diff to affected symbols"),
            make_tool("codegenome_trace", "Process trace from entrypoint"),
        ];
        std::future::ready(Ok(ListToolsResult {
            tools,
            next_cursor: None,
            meta: None,
        }))
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, McpError>> + Send + '_ {
        let result = dispatch_tool(self, &request);
        std::future::ready(Ok(result))
    }
}

fn dispatch_tool(tools: &CodegenomeTools, req: &CallToolRequestParams) -> CallToolResult {
    let args = req.arguments.as_ref();
    let text = match req.name.as_ref() {
        "codegenome_context" => {
            let file = arg_str(args, "file");
            let line = arg_u32(args, "line");
            tools.context(&file, line, 1)
        }
        "codegenome_impact" => {
            let file = arg_str(args, "file");
            let line = arg_u32(args, "line");
            tools.impact(&file, line)
        }
        "codegenome_detect_changes" => {
            let diff = arg_str(args, "diff");
            tools.detect(&diff)
        }
        "codegenome_trace" => {
            let ep = arg_str(args, "entrypoint");
            tools.trace(&ep)
        }
        _ => r#"{"error":"unknown tool"}"#.into(),
    };
    CallToolResult::success(vec![Content::text(text)])
}

fn arg_str(args: Option<&serde_json::Map<String, serde_json::Value>>, key: &str) -> String {
    args.and_then(|a| a.get(key))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn arg_u32(args: Option<&serde_json::Map<String, serde_json::Value>>, key: &str) -> u32 {
    args.and_then(|a| a.get(key))
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as u32
}

fn make_tool(name: &'static str, desc: &'static str) -> Tool {
    Tool {
        name: Cow::Borrowed(name),
        description: Some(Cow::Borrowed(desc)),
        input_schema: Arc::new(serde_json::Map::new()),
        title: None,
        output_schema: None,
        annotations: None,
        execution: None,
        icons: None,
        meta: None,
    }
}
