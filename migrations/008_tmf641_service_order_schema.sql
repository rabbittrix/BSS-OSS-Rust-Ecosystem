-- TMF641 Service Order Management API Schema
-- Service Orders table
CREATE TABLE
    IF NOT EXISTS service_orders (
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

-- Service Order Items table
CREATE TABLE
    IF NOT EXISTS service_order_items (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        order_id UUID NOT NULL REFERENCES service_orders (id) ON DELETE CASCADE,
        action VARCHAR(50) NOT NULL,
        service_specification_id UUID,
        service_id UUID,
        state VARCHAR(50) NOT NULL DEFAULT 'ACKNOWLEDGED',
        quantity INTEGER,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Service Order Related Parties table
CREATE TABLE
    IF NOT EXISTS service_order_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        order_id UUID NOT NULL REFERENCES service_orders (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_service_orders_state ON service_orders (state);

CREATE INDEX IF NOT EXISTS idx_service_orders_order_date ON service_orders (order_date);

CREATE INDEX IF NOT EXISTS idx_service_orders_external_id ON service_orders (external_id);

CREATE INDEX IF NOT EXISTS idx_service_order_items_order_id ON service_order_items (order_id);

CREATE INDEX IF NOT EXISTS idx_service_order_related_parties_order_id ON service_order_related_parties (order_id);

-- Comments
COMMENT ON TABLE service_orders IS 'TMF641 Service Orders - Service-level order management';

COMMENT ON TABLE service_order_items IS 'TMF641 Service Order Items - Individual items within a service order';

COMMENT ON TABLE service_order_related_parties IS 'TMF641 Related Parties - Parties related to service orders';