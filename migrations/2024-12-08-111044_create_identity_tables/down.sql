-- This file should undo anything in `up.sql`

-- Tabloları sil (ters sırada)
DROP TABLE IF EXISTS wallet_identities;
DROP TABLE IF EXISTS email_identities;
DROP TABLE IF EXISTS identities;

-- UUID extension'ını kaldır
DROP EXTENSION IF EXISTS "uuid-ossp";
