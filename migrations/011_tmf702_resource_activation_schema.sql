-- TMF702 Resource Activation & Configuration API Schema
-- Resource Activations table
CREATE TABLE
    IF NOT EXISTS resource_activations (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'PENDING',
        resource_id UUID,
        service_activation_id UUID,
        activation_date TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            completion_date TIMESTAMP
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

-- Resource Activation Configurations table
CREATE TABLE
    IF NOT EXISTS resource_activation_configurations (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        activation_id UUID NOT NULL REFERENCES resource_activations (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        value TEXT NOT NULL,
        description TEXT,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_resource_activations_state ON resource_activations (state);

CREATE INDEX IF NOT EXISTS idx_resource_activations_resource_id ON resource_activations (resource_id);

CREATE INDEX IF NOT EXISTS idx_resource_activations_service_activation_id ON resource_activations (service_activation_id);

CREATE INDEX IF NOT EXISTS idx_resource_activations_activation_date ON resource_activations (activation_date);

CREATE INDEX IF NOT EXISTS idx_resource_activation_configurations_activation_id ON resource_activation_configurations (activation_id);

-- Comments
COMMENT ON TABLE resource_activations IS 'TMF702 Resource Activations - Low-level provisioning of physical/virtual network elements';

COMMENT ON TABLE resource_activation_configurations IS 'TMF702 Configuration Parameters - Key-value configuration parameters for resource activations';