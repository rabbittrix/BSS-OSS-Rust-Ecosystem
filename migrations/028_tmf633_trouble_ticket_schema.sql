-- TMF633 Trouble Ticket Management schema
-- Create trouble_tickets table
CREATE TABLE
    IF NOT EXISTS trouble_tickets (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        href VARCHAR(500) NOT NULL,
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50) DEFAULT '1.0.0',
        status VARCHAR(50) NOT NULL DEFAULT 'SUBMITTED', -- SUBMITTED, ACKNOWLEDGED, IN_PROGRESS, RESOLVED, CLOSED, CANCELLED
        priority VARCHAR(50) NOT NULL DEFAULT 'MEDIUM', -- CRITICAL, HIGH, MEDIUM, LOW
        ticket_type VARCHAR(50) NOT NULL, -- SERVICE_ISSUE, BILLING_ISSUE, TECHNICAL_ISSUE, ACCOUNT_ISSUE, OTHER
        resolution TEXT,
        resolution_date TIMESTAMP
        WITH
            TIME ZONE,
            related_entity JSONB,
            customer_id UUID REFERENCES customers (id) ON DELETE SET NULL,
            assigned_to VARCHAR(255),
            tenant_id UUID REFERENCES tenants (id) ON DELETE CASCADE,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_trouble_tickets_status ON trouble_tickets (status);

CREATE INDEX IF NOT EXISTS idx_trouble_tickets_priority ON trouble_tickets (priority);

CREATE INDEX IF NOT EXISTS idx_trouble_tickets_customer_id ON trouble_tickets (customer_id);

CREATE INDEX IF NOT EXISTS idx_trouble_tickets_tenant_id ON trouble_tickets (tenant_id);

CREATE INDEX IF NOT EXISTS idx_trouble_tickets_assigned_to ON trouble_tickets (assigned_to);

CREATE INDEX IF NOT EXISTS idx_trouble_tickets_created_at ON trouble_tickets (created_at);

-- Add comments
COMMENT ON TABLE trouble_tickets IS 'TMF633 Trouble Ticket Management - Customer service tickets and issues';

COMMENT ON COLUMN trouble_tickets.status IS 'Ticket status: SUBMITTED, ACKNOWLEDGED, IN_PROGRESS, RESOLVED, CLOSED, CANCELLED';

COMMENT ON COLUMN trouble_tickets.priority IS 'Ticket priority: CRITICAL, HIGH, MEDIUM, LOW';

COMMENT ON COLUMN trouble_tickets.ticket_type IS 'Ticket type: SERVICE_ISSUE, BILLING_ISSUE, TECHNICAL_ISSUE, ACCOUNT_ISSUE, OTHER';