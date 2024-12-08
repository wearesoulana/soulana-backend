use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use crate::services::auth::AuthService;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct WalletAuthRequest {
    wallet_address: String,
}

#[derive(Deserialize)]
pub struct SignatureVerifyRequest {
    wallet_address: String,
    signature: String,
    nonce: String,
}

#[derive(Deserialize)]
pub struct LinkWalletRequest {
    email: String,
    wallet_address: String,
}

#[derive(Deserialize)]
pub struct LinkEmailRequest {
    wallet_address: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    nonce: String,
}

#[derive(Serialize)]
pub struct IdentitiesResponse {
    email: Option<String>,
    wallet_address: Option<String>,
}

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/wallet", web::post().to(wallet_auth))
        .route("/verify", web::post().to(verify_signature))
        .route("/link/wallet", web::post().to(link_wallet))
        .route("/link/email", web::post().to(link_email))
        .route("/identities/{id}", web::get().to(get_identities))
}

async fn wallet_auth(
    req: web::Json<WalletAuthRequest>,
    auth_service: web::Data<AuthService>,
) -> HttpResponse {
    match auth_service.wallet_auth(&req.wallet_address).await {
        Ok(wallet_identity) => HttpResponse::Ok().json(AuthResponse {
            nonce: wallet_identity.nonce,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn verify_signature(
    req: web::Json<SignatureVerifyRequest>,
    auth_service: web::Data<AuthService>,
) -> HttpResponse {
    if auth_service.verify_signature(&req.wallet_address, &req.signature, &req.nonce) {
        match auth_service.update_nonce(&req.wallet_address).await {
            Ok(new_nonce) => HttpResponse::Ok().json(AuthResponse { nonce: new_nonce }),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

async fn link_wallet(
    req: web::Json<LinkWalletRequest>,
    auth_service: web::Data<AuthService>,
) -> HttpResponse {
    match auth_service.link_wallet_to_email(&req.email, &req.wallet_address).await {
        Ok(wallet_identity) => HttpResponse::Ok().json(AuthResponse {
            nonce: wallet_identity.nonce,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn link_email(
    req: web::Json<LinkEmailRequest>,
    auth_service: web::Data<AuthService>,
) -> HttpResponse {
    // Gerçek uygulamada şifreyi hash'lemeniz gerekir
    let password_hash = req.password.clone(); // Bu sadece örnek!
    
    match auth_service.link_email_to_wallet(&req.wallet_address, &req.email, &password_hash).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_identities(
    id: web::Path<Uuid>,
    auth_service: web::Data<AuthService>,
) -> HttpResponse {
    match auth_service.get_user_identities(id.into_inner()).await {
        Ok((email_id, wallet_id)) => HttpResponse::Ok().json(IdentitiesResponse {
            email: email_id.map(|e| e.email),
            wallet_address: wallet_id.map(|w| w.wallet_address),
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
} 