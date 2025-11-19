-- Resource Management Schema
-- Extends TMF639 Resource Inventory with capacity, reservations, and topology
-- Resource Capacity Management
-- Tracks capacity metrics (bandwidth, CPU, memory, storage, etc.) for resources
CREATE TABLE
    IF NOT EXISTS resource_capacities (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        resource_inventory_id UUID NOT NULL REFERENCES resource_inventories (id) ON DELETE CASCADE,
        capacity_type VARCHAR(100) NOT NULL, -- e.g., 'BANDWIDTH', 'CPU', 'MEMORY', 'STORAGE', 'CONNECTIONS'
        total_capacity NUMERIC(15, 2) NOT NULL, -- Total available capacity
        used_capacity NUMERIC(15, 2) NOT NULL DEFAULT 0, -- Currently used capacity
        reserved_capacity NUMERIC(15, 2) NOT NULL DEFAULT 0, -- Reserved capacity
        unit VARCHAR(50) NOT NULL, -- e.g., 'Mbps', 'GHz', 'GB', 'count'
        created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            updated_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            UNIQUE (resource_inventory_id, capacity_type)
    );

-- Indexes for resource capacities
CREATE INDEX IF NOT EXISTS idx_resource_capacities_resource_id ON resource_capacities (resource_inventory_id);

CREATE INDEX IF NOT EXISTS idx_resource_capacities_type ON resource_capacities (capacity_type);

-- Resource Reservations
-- Tracks resource reservations for future use with time windows
CREATE TABLE
    IF NOT EXISTS resource_reservations (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        resource_inventory_id UUID NOT NULL REFERENCES resource_inventories (id) ON DELETE CASCADE,
        reservation_name VARCHAR(255) NOT NULL,
        description TEXT,
        reservation_status VARCHAR(50) NOT NULL DEFAULT 'PENDING', -- PENDING, CONFIRMED, ACTIVE, COMPLETED, CANCELLED
        start_time TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            end_time TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            resource_order_id UUID REFERENCES resource_orders (id) ON DELETE SET NULL, -- Link to resource order if applicable
            service_order_id UUID REFERENCES service_orders (id) ON DELETE SET NULL, -- Link to service order if applicable
            reserved_by_party_id UUID, -- Party making the reservation
            capacity_requirements JSONB, -- JSON object with capacity requirements: {"BANDWIDTH": 1000, "CPU": 2}
            created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            updated_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            confirmed_at TIMESTAMP
        WITH
            TIME ZONE,
            cancelled_at TIMESTAMP
        WITH
            TIME ZONE,
            cancellation_reason TEXT
    );

-- Indexes for resource reservations
CREATE INDEX IF NOT EXISTS idx_resource_reservations_resource_id ON resource_reservations (resource_inventory_id);

CREATE INDEX IF NOT EXISTS idx_resource_reservations_status ON resource_reservations (reservation_status);

CREATE INDEX IF NOT EXISTS idx_resource_reservations_time_range ON resource_reservations (start_time, end_time);

CREATE INDEX IF NOT EXISTS idx_resource_reservations_order_id ON resource_reservations (resource_order_id);

CREATE INDEX IF NOT EXISTS idx_resource_reservations_service_order_id ON resource_reservations (service_order_id);

-- Network Topology
-- Manages network connections and relationships between resources
CREATE TABLE
    IF NOT EXISTS network_topology (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        source_resource_id UUID NOT NULL REFERENCES resource_inventories (id) ON DELETE CASCADE,
        target_resource_id UUID NOT NULL REFERENCES resource_inventories (id) ON DELETE CASCADE,
        connection_type VARCHAR(100) NOT NULL, -- e.g., 'PHYSICAL', 'LOGICAL', 'VIRTUAL', 'OVERLAY'
        relationship_type VARCHAR(100) NOT NULL, -- e.g., 'CONNECTED_TO', 'DEPENDS_ON', 'PARENT_OF', 'CHILD_OF', 'PEER'
        connection_status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE', -- ACTIVE, INACTIVE, PLANNED, FAILED
        bandwidth_mbps NUMERIC(15, 2), -- Connection bandwidth if applicable
        latency_ms NUMERIC(10, 2), -- Connection latency if applicable
        description TEXT,
        created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            updated_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            UNIQUE (
                source_resource_id,
                target_resource_id,
                relationship_type
            )
    );

-- Indexes for network topology
CREATE INDEX IF NOT EXISTS idx_network_topology_source ON network_topology (source_resource_id);

CREATE INDEX IF NOT EXISTS idx_network_topology_target ON network_topology (target_resource_id);

CREATE INDEX IF NOT EXISTS idx_network_topology_type ON network_topology (connection_type);

CREATE INDEX IF NOT EXISTS idx_network_topology_relationship ON network_topology (relationship_type);

CREATE INDEX IF NOT EXISTS idx_network_topology_status ON network_topology (connection_status);

-- Network Topology Attributes
-- Additional attributes/metadata for topology connections
CREATE TABLE
    IF NOT EXISTS network_topology_attributes (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        topology_id UUID NOT NULL REFERENCES network_topology (id) ON DELETE CASCADE,
        attribute_name VARCHAR(255) NOT NULL,
        attribute_value TEXT NOT NULL,
        attribute_type VARCHAR(50) NOT NULL DEFAULT 'STRING', -- STRING, NUMBER, BOOLEAN, JSON
        created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            UNIQUE (topology_id, attribute_name)
    );

-- Index for topology attributes
CREATE INDEX IF NOT EXISTS idx_network_topology_attributes_topology_id ON network_topology_attributes (topology_id);

-- Comments
COMMENT ON TABLE resource_capacities IS 'Resource Capacity Management - Tracks capacity metrics for resources';

COMMENT ON TABLE resource_reservations IS 'Resource Reservations - Tracks resource reservations with time windows';

COMMENT ON TABLE network_topology IS 'Network Topology - Manages network connections and relationships between resources';

COMMENT ON TABLE network_topology_attributes IS 'Network Topology Attributes - Additional metadata for topology connections';