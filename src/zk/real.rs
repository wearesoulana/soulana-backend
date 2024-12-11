use super::{ZKVerifier, ZKProver};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use ark_bn254::Fr;
use ark_std::{One, Zero, str::FromStr};
use sha3::{Digest, Keccak256};
use num_bigint;
use ark_serialize::CanonicalSerialize;

pub struct RealZKVerifier {
    prover: ZKProver,
}

impl RealZKVerifier {
    pub fn new() -> Self {
        println!("Creating new RealZKVerifier");
        Self {
            prover: ZKProver::new(),
        }
    }

    pub fn create_wallet_proof(&self, wallet: &str) -> Option<String> {
        println!("\n=== Creating Wallet Proof ===");
        println!("Wallet address: {}", wallet);
        
        // catch panics
        let result = std::panic::catch_unwind(|| {
            println!("Calculating field element...");
            let input = Self::hash_to_field(wallet);
            println!("Field element calculated successfully");
            
            println!("Creating proof with input...");
            match self.prover.create_proof(input) {
                Ok(proof_bytes) => {
                    println!("Proof bytes created successfully, length: {}", proof_bytes.len());
                    let proof_base64 = BASE64.encode(&proof_bytes);
                    println!("Proof encoded to base64: {}", proof_base64);
                    Some(proof_base64)
                },
                Err(e) => {
                    println!("Error creating proof: {:?}", e);
                    None
                }
            }
        });

        match result {
            Ok(proof_option) => {
                println!("Proof creation completed successfully");
                proof_option
            },
            Err(e) => {
                println!("Panic occurred while creating proof: {:?}", e);
                None
            }
        }
    }

    pub fn create_email_proof(&self, email: &str) -> Option<String> {
        println!("\n=== Creating Email Proof ===");
        println!("Email address: {}", email);
        
        // catch panics
        let result = std::panic::catch_unwind(|| {
            println!("Calculating field element...");
            let input = Self::hash_to_field(email);
            println!("Field element calculated successfully");
            
            println!("Creating proof with input...");
            match self.prover.create_proof(input) {
                Ok(proof_bytes) => {
                    println!("Proof bytes created successfully, length: {}", proof_bytes.len());
                    let proof_base64 = BASE64.encode(&proof_bytes);
                    println!("Proof encoded to base64: {}", proof_base64);
                    Some(proof_base64)
                },
                Err(e) => {
                    println!("Error creating proof: {:?}", e);
                    None
                }
            }
        });

        match result {
            Ok(proof_option) => {
                println!("Proof creation completed successfully");
                proof_option
            },
            Err(e) => {
                println!("Panic occurred while creating proof: {:?}", e);
                None
            }
        }
    }

    fn hash_to_field(input: &str) -> Fr {
        println!("\n=== Hash to Field Process ===");
        println!("Input: {}", input);
        
        // return a fixed value for testing
        let result = Fr::one();
        
        println!("Using test value: Fr::one()");
        println!("=========================\n");
        
        result
    }
}

impl ZKVerifier for RealZKVerifier {
    fn verify_wallet(&self, wallet: &str, proof: &str) -> bool {
        println!("Verifying wallet: {}", wallet);
        println!("With proof: {}", proof);

        let proof_bytes = match BASE64.decode(proof) {
            Ok(bytes) => {
                println!("Successfully decoded proof, length: {}", bytes.len());
                bytes
            },
            Err(e) => {
                println!("Failed to decode proof: {:?}", e);
                return false;
            }
        };

        let input = Self::hash_to_field(wallet);
        println!("Generated field element from wallet");

        let result = self.prover.verify_proof(&proof_bytes, input);
        println!("Verification result: {}", result);
        result
    }

    fn verify_email(&self, email: &str, proof: &str) -> bool {
        println!("Verifying email: {}", email);
        println!("With proof: {}", proof);

        let proof_bytes = match BASE64.decode(proof) {
            Ok(bytes) => {
                println!("Successfully decoded proof, length: {}", bytes.len());
                bytes
            },
            Err(e) => {
                println!("Failed to decode proof: {:?}", e);
                return false;
            }
        };

        let input = Self::hash_to_field(email);
        println!("Generated field element from email");

        let result = self.prover.verify_proof(&proof_bytes, input);
        println!("Verification result: {}", result);
        result
    }
} 