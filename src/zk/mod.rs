pub mod real;
pub mod mock;

use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_ff::PrimeField;
use ark_bn254::{Bn254, Fr};
use ark_groth16::{
    Groth16,
    Proof,
    ProvingKey,
    VerifyingKey,
};
use ark_snark::SNARK;
use ark_std::{One, Zero, UniformRand};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use hex;
use std::fs;
use std::path::Path;

const PROVING_KEY_PATH: &str = "keys/proving_key.bin";
const VERIFYING_KEY_PATH: &str = "keys/verifying_key.bin";

#[derive(Clone)]
pub struct ZKCircuit<F: PrimeField> {
    pub input: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for ZKCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        println!("Generating constraints for ZK circuit");
        
        // Input variable - this is the public input
        let input_var = cs.new_input_variable(|| self.input.ok_or(SynthesisError::AssignmentMissing))?;
        
        // Witness variable - this is the private input
        let witness = cs.new_witness_variable(|| {
            let input_value = self.input.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(input_value)
        })?;

        // Constraint: input_var = witness
        cs.enforce_constraint(lc!() + input_var, lc!() + witness, lc!() + witness)?;

        println!("Constraints generated successfully");
        Ok(())
    }
}

pub struct ZKProver {
    proving_key: Vec<u8>,
    verifying_key: Vec<u8>,
}

impl ZKProver {
    pub fn new() -> Self {
        println!("Creating new ZKProver");
        
        // create keys directory if it doesn't exist
        if !Path::new("keys").exists() {
            fs::create_dir("keys").expect("Failed to create keys directory");
        }

        // if keys exist, load them
        if Path::new(PROVING_KEY_PATH).exists() && Path::new(VERIFYING_KEY_PATH).exists() {
            println!("Loading existing keys");
            let proving_key = fs::read(PROVING_KEY_PATH).expect("Failed to read proving key");
            let verifying_key = fs::read(VERIFYING_KEY_PATH).expect("Failed to read verifying key");
            return Self {
                proving_key,
                verifying_key,
            };
        }

        // if keys don't exist, generate new ones
        println!("Generating new keys");
        let rng = &mut ark_std::rand::thread_rng();
        
        // use a fixed input value
        let input = Fr::from(1u64);
        let circuit = ZKCircuit::<Fr> { input: Some(input) };
        
        println!("Generating circuit setup parameters");
        let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng).unwrap();
        
        println!("Serializing proving key");
        let mut proving_key = Vec::new();
        pk.serialize_compressed(&mut proving_key).unwrap();

        println!("Serializing verifying key");
        let mut verifying_key = Vec::new();
        vk.serialize_compressed(&mut verifying_key).unwrap();

        // save keys to files
        fs::write(PROVING_KEY_PATH, &proving_key).expect("Failed to save proving key");
        fs::write(VERIFYING_KEY_PATH, &verifying_key).expect("Failed to save verifying key");

        println!("ZKProver created successfully");
        Self {
            proving_key,
            verifying_key,
        }
    }

    pub fn create_proof(&self, input: Fr) -> Result<Vec<u8>, SynthesisError> {
        println!("Creating proof for input");
        let rng = &mut ark_std::rand::thread_rng();
        
        println!("Deserializing proving key");
        let pk = match ProvingKey::<Bn254>::deserialize_compressed(&self.proving_key[..]) {
            Ok(key) => key,
            Err(e) => {
                println!("Failed to deserialize proving key: {:?}", e);
                return Err(SynthesisError::Unsatisfiable);
            }
        };

        let circuit = ZKCircuit { input: Some(input) };
        
        println!("Creating proof");
        let proof = match Groth16::<Bn254>::prove(&pk, circuit, rng) {
            Ok(p) => p,
            Err(e) => {
                println!("Failed to create proof: {:?}", e);
                return Err(e);
            }
        };
        
        println!("Serializing proof");
        let mut proof_bytes = Vec::new();
        proof.serialize_compressed(&mut proof_bytes).unwrap();
        
        println!("Proof created successfully");
        Ok(proof_bytes)
    }

    pub fn verify_proof(&self, proof_bytes: &[u8], public_input: Fr) -> bool {
        println!("Starting proof verification");
        println!("Proof bytes length: {}", proof_bytes.len());
        println!("Public input (raw): {:?}", public_input);
        
        let mut bytes = Vec::new();
        public_input.serialize_compressed(&mut bytes).unwrap();
        println!("Public input (hex): 0x{}", hex::encode(&bytes));
        
        println!("Deserializing verifying key");
        let vk = match VerifyingKey::<Bn254>::deserialize_compressed(&self.verifying_key[..]) {
            Ok(key) => {
                println!("Successfully deserialized verifying key");
                key
            },
            Err(e) => {
                println!("Failed to deserialize verifying key: {:?}", e);
                return false;
            }
        };

        println!("Deserializing proof");
        let proof = match Proof::<Bn254>::deserialize_compressed(proof_bytes) {
            Ok(p) => {
                println!("Successfully deserialized proof");
                p
            },
            Err(e) => {
                println!("Failed to deserialize proof: {:?}", e);
                return false;
            }
        };

        println!("Verifying proof with public input");
        match Groth16::<Bn254>::verify(&vk, &[public_input], &proof) {
            Ok(result) => {
                println!("Proof verification completed: {}", result);
                result
            },
            Err(e) => {
                println!("Failed to verify proof: {:?}", e);
                false
            }
        }
    }
}

pub trait ZKVerifier {
    fn verify_wallet(&self, wallet: &str, proof: &str) -> bool;
    fn verify_email(&self, email: &str, proof: &str) -> bool;
} 