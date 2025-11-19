-- TMF639 Resource Inventory Management API Schema
-- Resource Inventories table
CREATE TABLE
    IF NOT EXISTS resource_inventories (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'AVAILABLE',
        resource_type VARCHAR(100),
        resource_specification_id UUID,
        resource_id UUID,
        activation_date TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
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

-- Resource Inventory Related Parties table
CREATE TABLE
    IF NOT EXISTS resource_inventory_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        inventory_id UUID NOT NULL REFERENCES resource_inventories (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_resource_inventories_state ON resource_inventories (state);

CREATE INDEX IF NOT EXISTS idx_resource_inventories_name ON resource_inventories (name);

CREATE INDEX IF NOT EXISTS idx_resource_inventories_resource_type ON resource_inventories (resource_type);

CREATE INDEX IF NOT EXISTS idx_resource_inventories_resource_id ON resource_inventories (resource_id);

CREATE INDEX IF NOT EXISTS idx_resource_inventory_related_parties_inventory_id ON resource_inventory_related_parties (inventory_id);

-- Comments
COMMENT ON TABLE resource_inventories IS 'TMF639 Resource Inventories - Tracks physical and virtual network resources';

COMMENT ON TABLE resource_inventory_related_parties IS 'TMF639 Related Parties - Parties related to resource inventories';