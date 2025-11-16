-- TMF637 Product Inventory Management API Schema
-- Product Inventories table
CREATE TABLE
    IF NOT EXISTS product_inventories (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'AVAILABLE',
        product_specification_id UUID,
        product_offering_id UUID,
        quantity INTEGER,
        reserved_quantity INTEGER DEFAULT 0,
        activation_date TIMESTAMP
        WITH
            TIME ZONE,
            last_modified_date TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            href VARCHAR(500),
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Inventory Related Parties table
CREATE TABLE
    IF NOT EXISTS inventory_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        inventory_id UUID NOT NULL REFERENCES product_inventories (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_product_inventories_state ON product_inventories (state);

CREATE INDEX IF NOT EXISTS idx_product_inventories_product_spec ON product_inventories (product_specification_id);

CREATE INDEX IF NOT EXISTS idx_product_inventories_product_offering ON product_inventories (product_offering_id);

CREATE INDEX IF NOT EXISTS idx_inventory_related_parties_inventory_id ON inventory_related_parties (inventory_id);

-- Comments
COMMENT ON TABLE product_inventories IS 'TMF637 Product Inventories - Management of what the customer owns';

COMMENT ON TABLE inventory_related_parties IS 'TMF637 Inventory Related Parties - Customers or parties related to inventory';