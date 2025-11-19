-- TMF645 Resource Order Management API Schema
-- Resource Orders table
CREATE TABLE
    IF NOT EXISTS resource_orders (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'ACKNOWLEDGED',
        order_date TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            expected_completion_date TIMESTAMP
        WITH
            TIME ZONE,
            priority VARCHAR(50),
            external_id VARCHAR(255),
            href VARCHAR(500),
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Resource Order Items table
CREATE TABLE
    IF NOT EXISTS resource_order_items (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        order_id UUID NOT NULL REFERENCES resource_orders (id) ON DELETE CASCADE,
        action VARCHAR(50) NOT NULL,
        resource_specification_id UUID,
        resource_id UUID,
        state VARCHAR(50) NOT NULL DEFAULT 'ACKNOWLEDGED',
        quantity INTEGER,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Resource Order Related Parties table
CREATE TABLE
    IF NOT EXISTS resource_order_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        order_id UUID NOT NULL REFERENCES resource_orders (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_resource_orders_state ON resource_orders (state);

CREATE INDEX IF NOT EXISTS idx_resource_orders_order_date ON resource_orders (order_date);

CREATE INDEX IF NOT EXISTS idx_resource_orders_external_id ON resource_orders (external_id);

CREATE INDEX IF NOT EXISTS idx_resource_order_items_order_id ON resource_order_items (order_id);

CREATE INDEX IF NOT EXISTS idx_resource_order_related_parties_order_id ON resource_order_related_parties (order_id);

-- Comments
COMMENT ON TABLE resource_orders IS 'TMF645 Resource Orders - Network resource order management';

COMMENT ON TABLE resource_order_items IS 'TMF645 Resource Order Items - Individual items within a resource order';

COMMENT ON TABLE resource_order_related_parties IS 'TMF645 Related Parties - Parties related to resource orders';