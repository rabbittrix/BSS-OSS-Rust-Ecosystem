-- TMF622 Product Ordering Management API Schema
-- Product Orders table
CREATE TABLE
    IF NOT EXISTS product_orders (
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
            href VARCHAR(500),
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Order Items table
CREATE TABLE
    IF NOT EXISTS order_items (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        order_id UUID NOT NULL REFERENCES product_orders (id) ON DELETE CASCADE,
        action VARCHAR(50) NOT NULL,
        product_offering_id UUID,
        product_specification_id UUID,
        state VARCHAR(50) NOT NULL DEFAULT 'ACKNOWLEDGED',
        quantity INTEGER,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Related Parties table (for orders)
CREATE TABLE
    IF NOT EXISTS related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        order_id UUID NOT NULL REFERENCES product_orders (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_product_orders_state ON product_orders (state);

CREATE INDEX IF NOT EXISTS idx_product_orders_order_date ON product_orders (order_date);

CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items (order_id);

CREATE INDEX IF NOT EXISTS idx_related_parties_order_id ON related_parties (order_id);

-- Comments
COMMENT ON TABLE product_orders IS 'TMF622 Product Orders - Customer order management';

COMMENT ON TABLE order_items IS 'TMF622 Order Items - Individual items within a product order';

COMMENT ON TABLE related_parties IS 'TMF622 Related Parties - Customers or parties related to orders';