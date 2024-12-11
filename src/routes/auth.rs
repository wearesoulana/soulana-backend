use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use crate::services::auth::AuthService;
use crate::zk::{real::RealZKVerifier, ZKVerifier};

#[derive(Debug, Deserialize)]
pub struct WalletAuthRequest {
    pub wallet_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZKProofRequest {
    pub proof: String,
    pub public_inputs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct WalletZKAuthRequest {
    pub wallet_address: String,
    pub zk_proof: ZKProofRequest,
}

#[derive(Debug, Deserialize)]
pub struct EmailZKAuthRequest {
    pub email: String,
    pub zk_proof: ZKProofRequest,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProofRequest {
    pub wallet_address: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEmailProofRequest {
    pub email: String,
}

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/wallet", web::post().to(wallet_auth))
        .route("/zk/wallet", web::post().to(wallet_zk_auth))
        .route("/zk/email", web::post().to(email_zk_auth))
        .route("/zk/create-proof", web::post().to(create_wallet_proof))
        .route("/zk/create-email-proof", web::post().to(create_email_proof))
}

async fn wallet_auth(
    req: web::Json<WalletAuthRequest>,
    auth_service: web::Data<AuthService>,
) -> HttpResponse {
    println!("Received wallet auth request: {:?}", req);
    match auth_service.wallet_auth(&req.wallet_address).await {
        Ok(wallet_identity) => HttpResponse::Ok().json(wallet_identity),
        Err(e) => {
            println!("Error in wallet auth: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to authenticate wallet".to_string()
            })
        }
    }
}

async fn wallet_zk_auth(
    req: web::Json<WalletZKAuthRequest>,
    auth_service: web::Data<AuthService>,
    zk_verifier: web::Data<RealZKVerifier>,
) -> HttpResponse {
    println!("Received wallet ZK auth request: {:?}", req);

    let verifier = zk_verifier.get_ref();
    let verification_result = verifier.verify_wallet(&req.wallet_address, &req.zk_proof.proof);
    println!("ZK verification result: {}", verification_result);

    if verification_result {
        match auth_service.create_auth_token(&req.wallet_address).await {
            Ok((token, user_id)) => {
                println!("Auth token created successfully");
                HttpResponse::Ok().json(AuthResponse {
                    token,
                    user_id,
                })
            },
            Err(e) => {
                println!("Failed to create auth token: {:?}", e);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to create auth token".to_string()
                })
            }
        }
    } else {
        println!("ZK verification failed");
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "ZK proof verification failed".to_string()
        })
    }
}

async fn email_zk_auth(
    req: web::Json<EmailZKAuthRequest>,
    auth_service: web::Data<AuthService>,
    zk_verifier: web::Data<RealZKVerifier>,
) -> HttpResponse {
    println!("Received email ZK auth request: {:?}", req);

    let verifier = zk_verifier.get_ref();
    let verification_result = verifier.verify_email(&req.email, &req.zk_proof.proof);
    println!("ZK verification result: {}", verification_result);

    if verification_result {
        match auth_service.create_auth_token(&req.email).await {
            Ok((token, user_id)) => {
                println!("Auth token created successfully");
                HttpResponse::Ok().json(AuthResponse {
                    token,
                    user_id,
                })
            },
            Err(e) => {
                println!("Failed to create auth token: {:?}", e);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to create auth token".to_string()
                })
            }
        }
    } else {
        println!("ZK verification failed");
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "ZK proof verification failed".to_string()
        })
    }
}

async fn create_wallet_proof(
    req: web::Json<CreateProofRequest>,
    zk_verifier: web::Data<RealZKVerifier>,
) -> HttpResponse {
    println!("Received create proof request: {:?}", req);

    let verifier = zk_verifier.get_ref();
    match verifier.create_wallet_proof(&req.wallet_address) {
        Some(proof) => HttpResponse::Ok().json(ZKProofRequest {
            proof,
            public_inputs: vec![],
        }),
        None => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to create proof".to_string(),
        }),
    }
}

async fn create_email_proof(
    req: web::Json<CreateEmailProofRequest>,
    zk_verifier: web::Data<RealZKVerifier>,
) -> HttpResponse {
    println!("Received create email proof request: {:?}", req);

    let verifier = zk_verifier.get_ref();
    match verifier.create_email_proof(&req.email) {
        Some(proof) => HttpResponse::Ok().json(ZKProofRequest {
            proof,
            public_inputs: vec![],
        }),
        None => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to create proof".to_string(),
        }),
    }
} 