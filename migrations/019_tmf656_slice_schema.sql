-- TMF656 Slice Management Schema
CREATE TABLE
    IF NOT EXISTS network_slices (
        id UUID PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'PLANNED',
        slice_type VARCHAR(100) NOT NULL,
        activation_date TIMESTAMP
        WITH
            TIME ZONE,
            termination_date TIMESTAMP
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

-- SLA Parameters (stored as JSON for flexibility)
CREATE TABLE
    IF NOT EXISTS network_slice_sla_parameters (
        id UUID PRIMARY KEY,
        network_slice_id UUID NOT NULL REFERENCES network_slices (id) ON DELETE CASCADE,
        max_latency_ms INTEGER,
        min_throughput_mbps INTEGER,
        max_devices INTEGER,
        coverage_area VARCHAR(255),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (network_slice_id)
    );

-- Network Function References
CREATE TABLE
    IF NOT EXISTS network_slice_functions (
        id UUID PRIMARY KEY,
        network_slice_id UUID NOT NULL REFERENCES network_slices (id) ON DELETE CASCADE,
        function_id UUID NOT NULL,
        name VARCHAR(255) NOT NULL,
        function_type VARCHAR(100),
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

CREATE INDEX IF NOT EXISTS idx_network_slices_state ON network_slices (state);

CREATE INDEX IF NOT EXISTS idx_network_slices_type ON network_slices (slice_type);

CREATE INDEX IF NOT EXISTS idx_network_slices_activation_date ON network_slices (activation_date);

CREATE INDEX IF NOT EXISTS idx_network_slice_functions_slice_id ON network_slice_functions (network_slice_id);

CREATE INDEX IF NOT EXISTS idx_network_slice_functions_function_id ON network_slice_functions (function_id);