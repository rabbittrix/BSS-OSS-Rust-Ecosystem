-- TMF669 Identity & Credential Management API Schema
-- Identities table
CREATE TABLE
    IF NOT EXISTS identities (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'CREATED',
        identity_type VARCHAR(100),
        party_id UUID,
        oauth_client_id VARCHAR(255),
        oauth_client_secret VARCHAR(500),
        jwt_issuer VARCHAR(255),
        expiration_date TIMESTAMP
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

-- Identity Credentials table
CREATE TABLE
    IF NOT EXISTS identity_credentials (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        identity_id UUID NOT NULL REFERENCES identities (id) ON DELETE CASCADE,
        credential_type VARCHAR(50) NOT NULL,
        credential_value VARCHAR(500),
        created_date TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            expiration_date TIMESTAMP
        WITH
            TIME ZONE,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_identities_state ON identities (state);

CREATE INDEX IF NOT EXISTS idx_identities_identity_type ON identities (identity_type);

CREATE INDEX IF NOT EXISTS idx_identities_party_id ON identities (party_id);

CREATE INDEX IF NOT EXISTS idx_identities_oauth_client_id ON identities (oauth_client_id);

CREATE INDEX IF NOT EXISTS idx_identity_credentials_identity_id ON identity_credentials (identity_id);

CREATE INDEX IF NOT EXISTS idx_identity_credentials_credential_type ON identity_credentials (credential_type);

-- Comments
COMMENT ON TABLE identities IS 'TMF669 Identities - Handles digital identities, credentials, OAuth/JWT integration';

COMMENT ON TABLE identity_credentials IS 'TMF669 Credentials - Credentials associated with identities';