-- Ana kimlik tablosu
CREATE TABLE identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Email kimlik tablosu
CREATE TABLE email_identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identity_id UUID REFERENCES identities(id) ON DELETE CASCADE,
    email VARCHAR NOT NULL UNIQUE,
    email_verified BOOLEAN DEFAULT FALSE,
    password_hash VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Wallet kimlik tablosu
CREATE TABLE wallet_identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identity_id UUID REFERENCES identities(id) ON DELETE CASCADE,
    wallet_address VARCHAR NOT NULL UNIQUE,
    nonce VARCHAR NOT NULL,  -- MetaMask imzalama için
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- İndeksler
CREATE INDEX idx_email_identities_identity_id ON email_identities(identity_id);
CREATE INDEX idx_wallet_identities_identity_id ON wallet_identities(identity_id);

-- UUID extension'ını etkinleştir
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
