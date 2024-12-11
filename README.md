# Soulana Backend

A privacy-focused identity verification system built on Solana blockchain, utilizing Zero-Knowledge Proofs for secure wallet and email authentication.

## Features

- 🔐 Zero-Knowledge Proof System
  - Secure wallet address verification
  - Private email verification
  - Proof generation and verification
  - Key management system

- 🌐 Authentication System
  - JWT-based authentication
  - Dual verification (wallet + email)
  - Nonce-based signature verification
  - Identity linking capabilities

- 🔧 Core Features
  - RESTful API endpoints
  - PostgreSQL database integration
  - Actix-web framework
  - Health check monitoring

## API Endpoints

### Authentication
- POST `/auth/wallet/prove` - Generate ZK proof for wallet
- POST `/auth/wallet/verify` - Verify wallet ZK proof
- POST `/auth/email/prove` - Generate ZK proof for email
- POST `/auth/email/verify` - Verify email ZK proof

### Health Check
- GET `/health` - Check API health status

## Environment Variables

Create a `.env` file with:

```plaintext
DATABASE_URL=postgresql://username:password@localhost/soulana_db
RUST_LOG=debug
```

## Technology Stack

- 🦀 Rust (Backend)
- 🎁 Actix-web (Web Framework)
- 🔐 ark-groth16 (ZK Proof System)
- 🗄️ PostgreSQL (Database)
- 🔗 Solana (Blockchain Integration)

## Running the Server

```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

## Database Setup

1. Install diesel_cli:
```bash
cargo install diesel_cli --no-default-features --features postgres
```

2. Set up the database:
```bash
diesel setup
```

3. Run migrations:
```bash
diesel migration run
```

This will create all necessary tables including:
- Identity tables for wallet and email verification
- User management tables
- Project related tables