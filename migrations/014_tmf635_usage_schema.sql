-- TMF635 Usage Management API Schema
-- Usage records table
CREATE TABLE
    IF NOT EXISTS usages (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'CAPTURED',
        usage_type VARCHAR(100),
        usage_date TIMESTAMP
        WITH
            TIME ZONE,
            start_date TIMESTAMP
        WITH
            TIME ZONE,
            end_date TIMESTAMP
        WITH
            TIME ZONE,
            amount DECIMAL(15, 2),
            unit VARCHAR(50),
            product_offering_id UUID,
            rating_id UUID,
            href VARCHAR(500),
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Usage Related Parties table
CREATE TABLE
    IF NOT EXISTS usage_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        usage_id UUID NOT NULL REFERENCES usages (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_usages_state ON usages (state);

CREATE INDEX IF NOT EXISTS idx_usages_usage_type ON usages (usage_type);

CREATE INDEX IF NOT EXISTS idx_usages_usage_date ON usages (usage_date);

CREATE INDEX IF NOT EXISTS idx_usages_product_offering_id ON usages (product_offering_id);

CREATE INDEX IF NOT EXISTS idx_usage_related_parties_usage_id ON usage_related_parties (usage_id);

-- Comments
COMMENT ON TABLE usages IS 'TMF635 Usage Records - Tracks and queries usage (CDRs, event consumption)';

COMMENT ON TABLE usage_related_parties IS 'TMF635 Related Parties - Parties related to usage records';