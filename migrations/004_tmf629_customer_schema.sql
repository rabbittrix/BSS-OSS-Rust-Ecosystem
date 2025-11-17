-- TMF629 Customer Management API Schema
-- Customers table
CREATE TABLE
    IF NOT EXISTS customers (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'INITIAL',
        status VARCHAR(100),
        href VARCHAR(500),
        last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Customer Contact Mediums table
CREATE TABLE
    IF NOT EXISTS customer_contact_mediums (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        customer_id UUID NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
        medium_type VARCHAR(100) NOT NULL,
        preferred BOOLEAN DEFAULT FALSE,
        value VARCHAR(500) NOT NULL,
        contact_type VARCHAR(100),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Customer Related Parties table
CREATE TABLE
    IF NOT EXISTS customer_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        customer_id UUID NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_customers_state ON customers (state);

CREATE INDEX IF NOT EXISTS idx_customers_name ON customers (name);

CREATE INDEX IF NOT EXISTS idx_customer_contact_mediums_customer_id ON customer_contact_mediums (customer_id);

CREATE INDEX IF NOT EXISTS idx_customer_related_parties_customer_id ON customer_related_parties (customer_id);

-- Comments
COMMENT ON TABLE customers IS 'TMF629 Customers - Customer profiles and contact information';

COMMENT ON TABLE customer_contact_mediums IS 'TMF629 Customer Contact Mediums - Contact information for customers';

COMMENT ON TABLE customer_related_parties IS 'TMF629 Customer Related Parties - Parties related to customers';