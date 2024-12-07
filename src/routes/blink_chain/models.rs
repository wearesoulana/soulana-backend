use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::projects)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image: String,
    pub target: String,
    pub raised: String,
    pub min_donation: f64,
    pub wallet: String,
}

#[derive(Debug, Deserialize)]
pub struct DonationRequest {
    pub id: i32,
    pub account: String,
    pub amount: String,
}

#[derive(Debug, Serialize)]
pub struct ActionParameter {
    pub name: String,
    pub label: String,
    pub required: bool,
}

#[derive(Debug, Serialize)]
pub struct LinkedAction {
    pub action_type: String,
    pub label: String,
    pub href: String,
    pub parameters: Vec<ActionParameter>,
}

#[derive(Debug, Serialize)]
pub struct ActionLinks {
    pub actions: Vec<LinkedAction>,
}

#[derive(Debug, Serialize)]
pub struct ActionGetResponse {
    pub action_type: String,
    pub title: String,
    pub icon: String,
    pub description: String,
    pub label: String,
    pub links: ActionLinks,
}

#[derive(Debug, Serialize)]
pub struct ActionPostResponse {
    pub transaction_type: String,
    pub transaction: String,
    pub message: String,
} 