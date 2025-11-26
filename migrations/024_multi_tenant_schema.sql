-- Multi-tenant support schema
-- This migration adds tenant isolation capabilities

-- Create tenant_status enum
CREATE TYPE IF NOT EXISTS tenant_status AS ENUM ('ACTIVE', 'SUSPENDED', 'INACTIVE');

-- Create tenants table
CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    domain VARCHAR(255) UNIQUE,
    status tenant_status NOT NULL DEFAULT 'ACTIVE',
    config JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index on domain for fast lookups
CREATE INDEX IF NOT EXISTS idx_tenants_domain ON tenants(domain) WHERE domain IS NOT NULL;

-- Create index on status for filtering
CREATE INDEX IF NOT EXISTS idx_tenants_status ON tenants(status);

-- Add tenant_id column to key tables for tenant isolation
-- Note: This is a simplified approach. In production, you might want to use
-- row-level security (RLS) or separate schemas per tenant

-- Add tenant_id to catalogs
ALTER TABLE catalogs ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_catalogs_tenant_id ON catalogs(tenant_id);

-- Add tenant_id to product_offerings
ALTER TABLE product_offerings ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_product_offerings_tenant_id ON product_offerings(tenant_id);

-- Add tenant_id to customers
ALTER TABLE customers ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_customers_tenant_id ON customers(tenant_id);

-- Add tenant_id to product_orders
ALTER TABLE product_orders ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_product_orders_tenant_id ON product_orders(tenant_id);

-- Add tenant_id to identities
ALTER TABLE identities ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_identities_tenant_id ON identities(tenant_id);

-- Add tenant_id to audit_logs
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_audit_logs_tenant_id ON audit_logs(tenant_id);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_tenants_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for updated_at
DROP TRIGGER IF EXISTS trigger_update_tenants_updated_at ON tenants;
CREATE TRIGGER trigger_update_tenants_updated_at
    BEFORE UPDATE ON tenants
    FOR EACH ROW
    EXECUTE FUNCTION update_tenants_updated_at();

-- Add comments
COMMENT ON TABLE tenants IS 'Multi-tenant support: Stores tenant information and configuration';
COMMENT ON COLUMN tenants.id IS 'Unique tenant identifier';
COMMENT ON COLUMN tenants.name IS 'Tenant name (unique)';
COMMENT ON COLUMN tenants.domain IS 'Tenant domain for subdomain-based routing (optional, unique)';
COMMENT ON COLUMN tenants.status IS 'Tenant status: ACTIVE, SUSPENDED, or INACTIVE';
COMMENT ON COLUMN tenants.config IS 'Tenant-specific configuration (JSON)';
COMMENT ON COLUMN tenants.created_at IS 'Tenant creation timestamp';
COMMENT ON COLUMN tenants.updated_at IS 'Last update timestamp';

