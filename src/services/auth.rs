use crate::models::{Identity, WalletIdentity, NewIdentity, NewWalletIdentity, EmailIdentity, NewEmailIdentity};
use crate::schema::{identities, wallet_identities, email_identities};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;
use chrono::Utc;

pub struct AuthService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AuthService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    // MetaMask ile giriş/kayıt
    pub async fn wallet_auth(&self, wallet_addr: &str) -> Result<WalletIdentity, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        
        // Wallet adresi ile kullanıcıyı ara
        let wallet_identity = wallet_identities::table
            .filter(wallet_identities::wallet_address.eq(wallet_addr))
            .first::<WalletIdentity>(&mut conn)
            .optional()?;

        match wallet_identity {
            Some(identity) => Ok(identity),
            None => {
                // Yeni kullanıcı oluştur
                let now = Utc::now().naive_utc();
                let new_identity = NewIdentity {
                    created_at: now,
                    updated_at: now,
                };
                
                let identity: Identity = diesel::insert_into(identities::table)
                    .values(&new_identity)
                    .get_result(&mut conn)?;

                // Nonce oluştur
                let nonce_str = Uuid::new_v4().to_string();

                // Wallet identity oluştur
                let new_wallet = NewWalletIdentity {
                    identity_id: Some(identity.id),
                    wallet_address: wallet_addr,
                    nonce: &nonce_str,
                };

                diesel::insert_into(wallet_identities::table)
                    .values(&new_wallet)
                    .get_result(&mut conn)
            }
        }
    }

    // İmza doğrulama
    pub fn verify_signature(&self, _wallet_address: &str, _signature: &str, _nonce: &str) -> bool {
        // İmza doğrulama mantığı burada olacak
        // Web3.js veya ethers-rs kullanarak imzayı doğrula
        true // Şimdilik her zaman true dönüyor
    }

    // Nonce güncelleme
    pub async fn update_nonce(&self, addr: &str) -> Result<String, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        
        let new_nonce_str = Uuid::new_v4().to_string();
        
        diesel::update(wallet_identities::table)
            .filter(wallet_identities::wallet_address.eq(addr))
            .set(wallet_identities::nonce.eq(&new_nonce_str))
            .execute(&mut conn)?;

        Ok(new_nonce_str)
    }

    // Email hesabına MetaMask bağlama
    pub async fn link_wallet_to_email(&self, user_email: &str, wallet_addr: &str) -> Result<WalletIdentity, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        
        // Email ile ana kimliği bul
        let email_identity = email_identities::table
            .filter(email_identities::email.eq(user_email))
            .first::<EmailIdentity>(&mut conn)?;

        // Nonce oluştur
        let nonce_str = Uuid::new_v4().to_string();

        // Wallet identity oluştur
        let new_wallet = NewWalletIdentity {
            identity_id: email_identity.identity_id,
            wallet_address: wallet_addr,
            nonce: &nonce_str,
        };

        diesel::insert_into(wallet_identities::table)
            .values(&new_wallet)
            .get_result(&mut conn)
    }

    // MetaMask hesabına email bağlama
    pub async fn link_email_to_wallet(&self, wallet_addr: &str, user_email: &str, password_hash: &str) -> Result<EmailIdentity, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        
        // Wallet ile ana kimliği bul
        let wallet_identity = wallet_identities::table
            .filter(wallet_identities::wallet_address.eq(wallet_addr))
            .first::<WalletIdentity>(&mut conn)?;

        // Email identity oluştur
        let new_email = NewEmailIdentity {
            identity_id: wallet_identity.identity_id,
            email: user_email,
            password_hash,
        };

        diesel::insert_into(email_identities::table)
            .values(&new_email)
            .get_result(&mut conn)
    }

    // Kullanıcının tüm kimliklerini getir
    pub async fn get_user_identities(&self, user_id: Uuid) -> Result<(Option<EmailIdentity>, Option<WalletIdentity>), diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        
        let email_id = email_identities::table
            .filter(email_identities::identity_id.eq(user_id))
            .first::<EmailIdentity>(&mut conn)
            .optional()?;

        let wallet_id = wallet_identities::table
            .filter(wallet_identities::identity_id.eq(user_id))
            .first::<WalletIdentity>(&mut conn)
            .optional()?;

        Ok((email_id, wallet_id))
    }
} 