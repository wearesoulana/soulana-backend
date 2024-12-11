use std::collections::HashMap;
use std::sync::Mutex;
use super::ZKVerifier;

#[derive(Debug, Default)]
pub struct MockZKVerifier {
    valid_proofs: Mutex<HashMap<String, bool>>,
}

impl MockZKVerifier {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ZKVerifier for MockZKVerifier {
    fn verify_wallet(&self, _wallet: &str, _proof: &str) -> bool {
        // Test için her zaman true döndür
        true
    }

    fn verify_email(&self, _email: &str, _proof: &str) -> bool {
        // Test için her zaman true döndür
        true
    }
} 