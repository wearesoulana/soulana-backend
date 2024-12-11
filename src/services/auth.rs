use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::{WalletIdentity, EmailIdentity};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: String,
    exp: usize,
}

pub struct AuthService {
    pool: Pool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: Pool, jwt_secret: String) -> Self {
        Self { 
            pool,
            jwt_secret,
        }
    }

    pub async fn wallet_auth(&self, wallet_address: &str) -> Result<WalletIdentity, diesel::result::Error> {
        // Implement wallet auth logic
        Ok(WalletIdentity {
            id: Uuid::new_v4(),
            wallet_address: wallet_address.to_string(),
            nonce: "test_nonce".to_string(),
        })
    }

    pub fn verify_signature(&self, wallet_address: &str, signature: &str, nonce: &str) -> bool {
        // Implement signature verification logic
        true
    }

    pub async fn update_nonce(&self, wallet_address: &str) -> Result<String, diesel::result::Error> {
        // Implement nonce update logic
        Ok("new_nonce".to_string())
    }

    pub async fn link_wallet_to_email(&self, email: &str, wallet_address: &str) -> Result<WalletIdentity, diesel::result::Error> {
        // Implement wallet linking logic
        Ok(WalletIdentity {
            id: Uuid::new_v4(),
            wallet_address: wallet_address.to_string(),
            nonce: "test_nonce".to_string(),
        })
    }

    pub async fn link_email_to_wallet(&self, wallet_address: &str, email: &str, password_hash: &str) -> Result<(), diesel::result::Error> {
        // Implement email linking logic
        Ok(())
    }

    pub async fn get_user_identities(&self, user_id: Uuid) -> Result<(Option<EmailIdentity>, Option<WalletIdentity>), diesel::result::Error> {
        // Implement get identities logic
        Ok((None, None))
    }

    pub async fn create_auth_token(&self, identity: &str) -> Result<(String, String), jsonwebtoken::errors::Error> {
        let user_id = Uuid::new_v4().to_string();
        let claims = Claims {
            sub: identity.to_string(),
            user_id: user_id.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok((token, user_id))
    }
} 