use sha2::{Digest, Sha256};

/// Computes a hash based on the name, seed, and sequence number.
///
/// This function uses SHA256.
pub fn compute_hash(name: &str, seed: &[u8], seq: u64) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    hasher.update(seed);
    hasher.update(seq.to_string().as_bytes()); // Convert u64 to string for hashing

    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash_happy_path() {
        let name = "test";
        let seed = b"seed";
        let seq = 1;
        let expected_hex = "468460ee3c32ca9574f91f213853d0b0aece116aa74b71ab66bb7a9c558b2b7c";

        let result = compute_hash(name, seed, seq);
        let result_hex = hex::encode(&result);

        assert_eq!(result_hex, expected_hex, "Hash mismatch for happy path");
    }

    #[test]
    fn test_compute_hash_empty_name() {
        let name = "";
        let seed = b"seed";
        let seq = 1;
        let expected_hex = "df9ecf4c79e5ad77701cfc88c196632b353149d85810a381f469f8fc05dc1b92";

        let result = compute_hash(name, seed, seq);
        let result_hex = hex::encode(&result);

        assert_eq!(result_hex, expected_hex, "Hash mismatch for no name");
    }

    #[test]
    fn test_compute_hash_empty_seed() {
        let name = "test";
        let seed = b"";
        let seq = 1;
        let expected_hex = "1b4f0e9851971998e732078544c96b36c3d01cedf7caa332359d6f1d83567014";

        let result = compute_hash(name, seed, seq);
        let result_hex = hex::encode(&result);

        assert_eq!(result_hex, expected_hex, "Hash mismatch for empty seed");
    }
}
