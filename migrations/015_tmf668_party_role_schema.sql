-- TMF668 Party Role Management API Schema
-- Party Roles table
CREATE TABLE
    IF NOT EXISTS party_roles (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'INITIALIZED',
        role VARCHAR(100) NOT NULL,
        party_type VARCHAR(50),
        engagement_date TIMESTAMP
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

-- Party Role Contact Mediums table
CREATE TABLE
    IF NOT EXISTS party_role_contact_mediums (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        party_role_id UUID NOT NULL REFERENCES party_roles (id) ON DELETE CASCADE,
        medium_type VARCHAR(100) NOT NULL,
        value VARCHAR(500) NOT NULL,
        preferred BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Party Role Related Parties table
CREATE TABLE
    IF NOT EXISTS party_role_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        party_role_id UUID NOT NULL REFERENCES party_roles (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_party_roles_state ON party_roles (state);

CREATE INDEX IF NOT EXISTS idx_party_roles_role ON party_roles (role);

CREATE INDEX IF NOT EXISTS idx_party_roles_party_type ON party_roles (party_type);

CREATE INDEX IF NOT EXISTS idx_party_role_contact_mediums_party_role_id ON party_role_contact_mediums (party_role_id);

CREATE INDEX IF NOT EXISTS idx_party_role_related_parties_party_role_id ON party_role_related_parties (party_role_id);

-- Comments
COMMENT ON TABLE party_roles IS 'TMF668 Party Roles - Manages parties, organizations, roles, partners';

COMMENT ON TABLE party_role_contact_mediums IS 'TMF668 Contact Mediums - Contact information for party roles';

COMMENT ON TABLE party_role_related_parties IS 'TMF668 Related Parties - Parties related to party roles';