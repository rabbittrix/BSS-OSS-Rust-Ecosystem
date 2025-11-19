-- TMF638 Service Inventory Management API Schema
-- Service Inventories table
CREATE TABLE
    IF NOT EXISTS service_inventories (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
        service_specification_id UUID,
        service_id UUID,
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

-- Service Inventory Related Parties table
CREATE TABLE
    IF NOT EXISTS service_inventory_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        inventory_id UUID NOT NULL REFERENCES service_inventories (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_service_inventories_state ON service_inventories (state);

CREATE INDEX IF NOT EXISTS idx_service_inventories_name ON service_inventories (name);

CREATE INDEX IF NOT EXISTS idx_service_inventories_service_id ON service_inventories (service_id);

CREATE INDEX IF NOT EXISTS idx_service_inventory_related_parties_inventory_id ON service_inventory_related_parties (inventory_id);

-- Comments
COMMENT ON TABLE service_inventories IS 'TMF638 Service Inventories - Management of provisioned services and service instances';

COMMENT ON TABLE service_inventory_related_parties IS 'TMF638 Related Parties - Parties related to service inventories';