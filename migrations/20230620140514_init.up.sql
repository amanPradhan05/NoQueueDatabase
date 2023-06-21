-- Add up migration script here
-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS items (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        barcodestring  BIGINT NOT NULL,
        title VARCHAR(255) NOT NULL UNIQUE,
        price INT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
