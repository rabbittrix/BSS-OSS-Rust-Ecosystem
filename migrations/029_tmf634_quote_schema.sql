-- TMF634 Quote Management API Schema
-- Quotes table
CREATE TABLE
    IF NOT EXISTS quotes (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50) DEFAULT '1.0.0',
        state VARCHAR(50) NOT NULL DEFAULT 'IN_PROGRESS',
        quote_date TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            valid_until TIMESTAMP
        WITH
            TIME ZONE,
            total_price JSONB,
            expected_order_date TIMESTAMP
        WITH
            TIME ZONE,
            href VARCHAR(500),
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Quote Items table
CREATE TABLE
    IF NOT EXISTS quote_items (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        quote_id UUID NOT NULL REFERENCES quotes (id) ON DELETE CASCADE,
        product_offering_id UUID,
        product_specification_id UUID,
        quantity INTEGER,
        unit_price JSONB,
        total_price JSONB,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Quote Related Parties table
CREATE TABLE
    IF NOT EXISTS quote_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        quote_id UUID NOT NULL REFERENCES quotes (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_quotes_state ON quotes (state);

CREATE INDEX IF NOT EXISTS idx_quotes_quote_date ON quotes (quote_date);

CREATE INDEX IF NOT EXISTS idx_quotes_valid_until ON quotes (valid_until);

CREATE INDEX IF NOT EXISTS idx_quote_items_quote_id ON quote_items (quote_id);

CREATE INDEX IF NOT EXISTS idx_quote_related_parties_quote_id ON quote_related_parties (quote_id);

-- Comments
COMMENT ON TABLE quotes IS 'TMF634 Quotes - Product and service price quotes';

COMMENT ON TABLE quote_items IS 'TMF634 Quote Items - Individual items within a quote';

COMMENT ON TABLE quote_related_parties IS 'TMF634 Quote Related Parties - Customers or parties related to quotes';