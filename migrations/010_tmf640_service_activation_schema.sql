-- TMF640 Service Activation & Configuration API Schema
-- Service Activations table
CREATE TABLE
    IF NOT EXISTS service_activations (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'PENDING',
        service_id UUID,
        service_order_id UUID,
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

-- Service Activation Configurations table
CREATE TABLE
    IF NOT EXISTS service_activation_configurations (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        activation_id UUID NOT NULL REFERENCES service_activations (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        value TEXT NOT NULL,
        description TEXT,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_service_activations_state ON service_activations (state);

CREATE INDEX IF NOT EXISTS idx_service_activations_service_id ON service_activations (service_id);

CREATE INDEX IF NOT EXISTS idx_service_activations_service_order_id ON service_activations (service_order_id);

CREATE INDEX IF NOT EXISTS idx_service_activations_activation_date ON service_activations (activation_date);

CREATE INDEX IF NOT EXISTS idx_service_activation_configurations_activation_id ON service_activation_configurations (activation_id);

-- Comments
COMMENT ON TABLE service_activations IS 'TMF640 Service Activations - Service provisioning actions on network elements';

COMMENT ON TABLE service_activation_configurations IS 'TMF640 Configuration Parameters - Key-value configuration parameters for service activations';