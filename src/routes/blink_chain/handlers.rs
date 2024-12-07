use actix_web::{get, post, options, web, HttpResponse, Responder, HttpRequest};
use diesel::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::Message,
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    signer::keypair::Keypair,
};
use crate::DbPool;
use super::models::*;

#[get("")]
pub async fn get_project(
    pool: web::Data<DbPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
    req: HttpRequest,
) -> impl Responder {
    use crate::schema::projects::dsl::*;

    let project_id = match query.get("id").and_then(|val| val.parse::<i32>().ok()) {
        Some(pid) => pid,
        None => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid project ID"
        })),
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Database connection error"
        })),
    };

    let project = match projects
        .filter(id.eq(project_id))
        .first::<Project>(&mut conn) {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Project not found"
        })),
    };

    let base_href = format!("{}/api/blink-chain?", req.connection_info().host());
    
    let response = ActionGetResponse {
        action_type: "action".to_string(),
        title: project.title,
        icon: project.image,
        description: project.description,
        label: "Transfer".to_string(),
        links: ActionLinks {
            actions: vec![LinkedAction {
                action_type: "transaction".to_string(),
                label: "Send".to_string(),
                href: format!("{}id={}&amount={{amount}}", base_href, project.id),
                parameters: vec![ActionParameter {
                    name: "amount".to_string(),
                    label: "Amount".to_string(),
                    required: true,
                }],
            }],
        },
    };

    HttpResponse::Ok().json(response)
}

#[post("")]
pub async fn process_donation(
    pool: web::Data<DbPool>,
    donation: web::Json<DonationRequest>,
) -> impl Responder {
    use crate::schema::projects::dsl::*;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Database connection error"
        })),
    };

    // Find project
    let project = match projects
        .filter(id.eq(donation.id))
        .first::<Project>(&mut conn) {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Project not found"
        })),
    };

    // Parse account
    let account = match Pubkey::try_from(donation.account.as_str()) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid account provided"
        })),
    };

    // Parse project wallet
    let project_wallet = match Pubkey::try_from(project.wallet.as_str()) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid project wallet"
        })),
    };

    // Validate amount
    let donation_amount = match donation.amount.parse::<f64>() {
        Ok(amount) => amount,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid donation amount"
        })),
    };

    if donation_amount < project.min_donation {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Donation amount must be at least {} SOL", project.min_donation)
        }));
    }

    // Setup Solana client
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let donation_lamports = (donation_amount * 1_000_000_000.0) as u64;

    // Check balance
    match rpc_client.get_balance(&account) {
        Ok(balance) if balance < donation_lamports => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Insufficient balance for donation"
            }))
        }
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to check balance"
        })),
        _ => {}
    }

    // Create transfer instruction
    let instruction = system_instruction::transfer(&account, &project_wallet, donation_lamports);

    // Get recent blockhash
    let recent_blockhash = match rpc_client.get_latest_blockhash() {
        Ok(blockhash) => blockhash,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get blockhash"
        })),
    };

    // Create transaction
    let message = Message::new(&[instruction], Some(&account));
    let transaction = Transaction::new::<[&Keypair; 0]>(&[], message, recent_blockhash);

    let response = ActionPostResponse {
        transaction_type: "transaction".to_string(),
        transaction: bs58::encode(transaction.message_data()).into_string(),
        message: format!("Donating {} SOL to {}", donation_amount, project.title),
    };

    HttpResponse::Ok().json(response)
}

#[options("")]
pub async fn options() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Access-Control-Allow-Methods", "GET, POST, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .finish()
} 