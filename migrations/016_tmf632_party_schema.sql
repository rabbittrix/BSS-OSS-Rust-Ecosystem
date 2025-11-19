-- TMF632 Party Management API Schema
-- Parties table
CREATE TABLE
    IF NOT EXISTS parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'INITIALIZED',
        party_type VARCHAR(50) NOT NULL,
        registration_date TIMESTAMP
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

-- Party Contact Mediums table
CREATE TABLE
    IF NOT EXISTS party_contact_mediums (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        party_id UUID NOT NULL REFERENCES parties (id) ON DELETE CASCADE,
        medium_type VARCHAR(100) NOT NULL,
        value VARCHAR(500) NOT NULL,
        preferred BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Party Related Parties table
CREATE TABLE
    IF NOT EXISTS party_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        party_id UUID NOT NULL REFERENCES parties (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Party Accounts table
CREATE TABLE
    IF NOT EXISTS party_accounts (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        party_id UUID NOT NULL REFERENCES parties (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Party Characteristics table
CREATE TABLE
    IF NOT EXISTS party_characteristics (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        party_id UUID NOT NULL REFERENCES parties (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        value TEXT NOT NULL,
        value_type VARCHAR(50),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_parties_state ON parties (state);

CREATE INDEX IF NOT EXISTS idx_parties_party_type ON parties (party_type);

CREATE INDEX IF NOT EXISTS idx_party_contact_mediums_party_id ON party_contact_mediums (party_id);

CREATE INDEX IF NOT EXISTS idx_party_related_parties_party_id ON party_related_parties (party_id);

CREATE INDEX IF NOT EXISTS idx_party_accounts_party_id ON party_accounts (party_id);

CREATE INDEX IF NOT EXISTS idx_party_characteristics_party_id ON party_characteristics (party_id);

-- Comments
COMMENT ON TABLE parties IS 'TMF632 Parties - Manages individuals, organizations, account-level attributes';

COMMENT ON TABLE party_contact_mediums IS 'TMF632 Contact Mediums - Contact information for parties';

COMMENT ON TABLE party_related_parties IS 'TMF632 Related Parties - Parties related to parties';

COMMENT ON TABLE party_accounts IS 'TMF632 Accounts - Account references for parties';

COMMENT ON TABLE party_characteristics IS 'TMF632 Characteristics - Attributes for parties';

