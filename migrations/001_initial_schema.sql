-- BSS/OSS Rust - Initial Database Schema
-- Run this script to set up the database tables for TMF620 Product Catalog Management
-- Create catalogs table
CREATE TABLE
    IF NOT EXISTS catalogs (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        lifecycle_status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
        href VARCHAR(255),
        last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            valid_for_start TIMESTAMP
        WITH
            TIME ZONE,
            valid_for_end TIMESTAMP
        WITH
            TIME ZONE,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Create product_offerings table
CREATE TABLE
    IF NOT EXISTS product_offerings (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        lifecycle_status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
        href VARCHAR(255),
        last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            valid_for_start TIMESTAMP
        WITH
            TIME ZONE,
            valid_for_end TIMESTAMP
        WITH
            TIME ZONE,
            is_sellable BOOLEAN DEFAULT false,
            is_bundle BOOLEAN DEFAULT false,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_catalogs_lifecycle_status ON catalogs (lifecycle_status);

CREATE INDEX IF NOT EXISTS idx_catalogs_name ON catalogs (name);

CREATE INDEX IF NOT EXISTS idx_product_offerings_lifecycle_status ON product_offerings (lifecycle_status);

CREATE INDEX IF NOT EXISTS idx_product_offerings_name ON product_offerings (name);

CREATE INDEX IF NOT EXISTS idx_product_offerings_is_sellable ON product_offerings (is_sellable);

-- Add comments for documentation
COMMENT ON TABLE catalogs IS 'Product catalogs following TMF620 specification';

COMMENT ON TABLE product_offerings IS 'Product offerings following TMF620 specification';