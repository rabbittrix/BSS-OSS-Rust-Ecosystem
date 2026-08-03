-- TMF633 Service Catalog, TMF634 Resource Catalog, TMF679 Product Offering Qualification

CREATE TABLE IF NOT EXISTS service_catalogs (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    lifecycle_status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    href TEXT,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS service_specifications (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    lifecycle_status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    href TEXT,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    category VARCHAR(100),
    is_bundle BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS resource_catalogs (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    lifecycle_status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    href TEXT,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS resource_catalog_specifications (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    lifecycle_status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    href TEXT,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    category VARCHAR(100),
    is_bundle BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS product_offering_qualifications (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    href TEXT,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    state VARCHAR(50) NOT NULL DEFAULT 'DONE',
    provide_alternative BOOLEAN DEFAULT FALSE,
    provide_unavailability_reason BOOLEAN DEFAULT FALSE,
    qualification_result VARCHAR(50) NOT NULL,
    product_offering_id UUID NOT NULL,
    product_offering_name VARCHAR(255) NOT NULL,
    eligibility_reason TEXT,
    customer_id UUID,
    requested_date TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_service_catalogs_name ON service_catalogs (name);
CREATE INDEX IF NOT EXISTS idx_service_specs_name ON service_specifications (name);
CREATE INDEX IF NOT EXISTS idx_resource_catalogs_name ON resource_catalogs (name);
CREATE INDEX IF NOT EXISTS idx_resource_catalog_specs_name ON resource_catalog_specifications (name);
CREATE INDEX IF NOT EXISTS idx_poq_customer ON product_offering_qualifications (customer_id);
CREATE INDEX IF NOT EXISTS idx_poq_offering ON product_offering_qualifications (product_offering_id);

COMMENT ON TABLE service_catalogs IS 'TMF633 Service Catalog';
COMMENT ON TABLE service_specifications IS 'TMF633 Service Specification';
COMMENT ON TABLE resource_catalogs IS 'TMF634 Resource Catalog';
COMMENT ON TABLE resource_catalog_specifications IS 'TMF634 Resource Specification';
COMMENT ON TABLE product_offering_qualifications IS 'TMF679 Product Offering Qualification';
