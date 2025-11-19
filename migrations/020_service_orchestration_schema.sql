-- Service Orchestration Workflow Contexts
-- This table stores the workflow context for service lifecycle orchestration
CREATE TABLE
    IF NOT EXISTS service_workflow_contexts (
        service_order_id UUID PRIMARY KEY,
        state VARCHAR(50) NOT NULL,
        context_data JSONB NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            updated_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            completed_at TIMESTAMP
        WITH
            TIME ZONE,
            error TEXT,
            FOREIGN KEY (service_order_id) REFERENCES service_orders (id) ON DELETE CASCADE
    );

-- Index for querying by state
CREATE INDEX IF NOT EXISTS idx_service_workflow_contexts_state ON service_workflow_contexts (state);

-- Index for querying by updated_at (for processing workflows)
CREATE INDEX IF NOT EXISTS idx_service_workflow_contexts_updated_at ON service_workflow_contexts (updated_at);

-- Service Dependency Graph
-- This table stores service specification dependencies
CREATE TABLE
    IF NOT EXISTS service_dependencies (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        service_specification_id UUID NOT NULL,
        depends_on_specification_id UUID NOT NULL,
        dependency_type VARCHAR(50) NOT NULL DEFAULT 'REQUIRES_ACTIVE',
        required BOOLEAN NOT NULL DEFAULT true,
        created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            UNIQUE (
                service_specification_id,
                depends_on_specification_id
            )
    );

-- Index for querying dependencies
CREATE INDEX IF NOT EXISTS idx_service_dependencies_service_spec ON service_dependencies (service_specification_id);

-- Index for querying dependents
CREATE INDEX IF NOT EXISTS idx_service_dependencies_depends_on ON service_dependencies (depends_on_specification_id);

-- Service Specification States
-- This table tracks the state of service specifications (provisioned, active, etc.)
CREATE TABLE
    IF NOT EXISTS service_specification_states (
        service_specification_id UUID PRIMARY KEY,
        service_id UUID,
        state VARCHAR(50) NOT NULL DEFAULT 'NOT_PROVISIONED',
        created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW (),
            updated_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT NOW ()
    );

-- Index for querying by state
CREATE INDEX IF NOT EXISTS idx_service_spec_states_state ON service_specification_states (state);