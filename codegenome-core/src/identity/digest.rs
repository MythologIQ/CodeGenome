use crate::identity::UorAddress;

/// Canonical serialization -> BLAKE3 digest -> UorAddress.
/// Pure function. No state. No side effects.
pub fn address_of(canonical_bytes: &[u8]) -> UorAddress {
    let hash = blake3::hash(canonical_bytes);
    UorAddress(*hash.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_bytes_same_address() {
        let bytes = b"hello world";
        assert_eq!(address_of(bytes), address_of(bytes));
    }

    #[test]
    fn different_bytes_different_address() {
        assert_ne!(address_of(b"hello"), address_of(b"world"));
    }

    #[test]
    fn empty_bytes_deterministic() {
        let a = address_of(b"");
        let b = address_of(b"");
        assert_eq!(a, b);
    }
}
