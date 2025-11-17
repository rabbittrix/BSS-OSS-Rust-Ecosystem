-- TMF678 Customer Bill Management API Schema
-- Customer Bills table
CREATE TABLE
    IF NOT EXISTS customer_bills (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'PENDING',
        bill_date TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            due_date TIMESTAMP
        WITH
            TIME ZONE,
            total_amount_value DECIMAL(15, 2),
            total_amount_unit VARCHAR(10) DEFAULT 'USD',
            tax_included BOOLEAN DEFAULT FALSE,
            href VARCHAR(500),
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Bill Items table
CREATE TABLE
    IF NOT EXISTS bill_items (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        bill_id UUID NOT NULL REFERENCES customer_bills (id) ON DELETE CASCADE,
        description TEXT NOT NULL,
        amount_value DECIMAL(15, 2) NOT NULL,
        amount_unit VARCHAR(10) DEFAULT 'USD',
        quantity INTEGER,
        product_offering_id UUID,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Bill Related Parties table
CREATE TABLE
    IF NOT EXISTS bill_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        bill_id UUID NOT NULL REFERENCES customer_bills (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_customer_bills_state ON customer_bills (state);

CREATE INDEX IF NOT EXISTS idx_customer_bills_bill_date ON customer_bills (bill_date);

CREATE INDEX IF NOT EXISTS idx_customer_bills_due_date ON customer_bills (due_date);

CREATE INDEX IF NOT EXISTS idx_bill_items_bill_id ON bill_items (bill_id);

CREATE INDEX IF NOT EXISTS idx_bill_related_parties_bill_id ON bill_related_parties (bill_id);

-- Comments
COMMENT ON TABLE customer_bills IS 'TMF678 Customer Bills - Bills and billing structures';

COMMENT ON TABLE bill_items IS 'TMF678 Bill Items - Individual items within a bill';

COMMENT ON TABLE bill_related_parties IS 'TMF678 Bill Related Parties - Parties related to bills';