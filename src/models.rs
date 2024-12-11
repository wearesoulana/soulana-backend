use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletIdentity {
    pub id: Uuid,
    pub wallet_address: String,
    pub nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailIdentity {
    pub id: Uuid,
    pub email: String,
} 