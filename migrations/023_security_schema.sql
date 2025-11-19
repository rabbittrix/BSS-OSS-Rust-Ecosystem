-- Security System Schema
-- OAuth 2.0 / OIDC, MFA, RBAC, and Audit Logging

-- OAuth Clients
CREATE TABLE IF NOT EXISTS oauth_clients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id VARCHAR(255) NOT NULL UNIQUE,
    client_secret_hash VARCHAR(255) NOT NULL,
    redirect_uris TEXT[] NOT NULL,
    grant_types TEXT[] NOT NULL,
    scopes TEXT NOT NULL,
    identity_id UUID NOT NULL REFERENCES identities(id) ON DELETE CASCADE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_oauth_clients_client_id ON oauth_clients(client_id);
CREATE INDEX idx_oauth_clients_identity_id ON oauth_clients(identity_id);

-- Authorization Codes
CREATE TABLE IF NOT EXISTS authorization_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(255) NOT NULL UNIQUE,
    client_id VARCHAR(255) NOT NULL,
    user_id UUID NOT NULL,
    redirect_uri VARCHAR(2048) NOT NULL,
    scopes TEXT[] NOT NULL,
    code_challenge VARCHAR(255),
    code_challenge_method VARCHAR(10),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_authorization_codes_code ON authorization_codes(code);
CREATE INDEX idx_authorization_codes_client_id ON authorization_codes(client_id);
CREATE INDEX idx_authorization_codes_expires_at ON authorization_codes(expires_at);

-- Access Tokens
CREATE TABLE IF NOT EXISTS access_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(512) NOT NULL UNIQUE,
    token_type VARCHAR(50) NOT NULL DEFAULT 'Bearer',
    expires_in INTEGER NOT NULL,
    refresh_token VARCHAR(512),
    refresh_expires_at TIMESTAMP WITH TIME ZONE,
    scope TEXT NOT NULL,
    client_id VARCHAR(255) NOT NULL,
    user_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_access_tokens_token ON access_tokens(token);
CREATE INDEX idx_access_tokens_refresh_token ON access_tokens(refresh_token);
CREATE INDEX idx_access_tokens_client_id ON access_tokens(client_id);
CREATE INDEX idx_access_tokens_user_id ON access_tokens(user_id);
CREATE INDEX idx_access_tokens_expires_at ON access_tokens(expires_at);

-- MFA Configurations
CREATE TABLE IF NOT EXISTS mfa_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identity_id UUID NOT NULL REFERENCES identities(id) ON DELETE CASCADE,
    method VARCHAR(50) NOT NULL, -- TOTP, SMS, EMAIL
    secret TEXT, -- Encrypted secret for TOTP
    phone_number VARCHAR(50), -- For SMS
    email VARCHAR(255), -- For Email
    backup_codes TEXT[], -- Hashed backup codes
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_mfa_configs_identity_id ON mfa_configs(identity_id);
CREATE INDEX idx_mfa_configs_method ON mfa_configs(method);
CREATE INDEX idx_mfa_configs_enabled ON mfa_configs(is_enabled);

-- MFA Challenges
CREATE TABLE IF NOT EXISTS mfa_challenges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identity_id UUID NOT NULL REFERENCES identities(id) ON DELETE CASCADE,
    method VARCHAR(50) NOT NULL,
    challenge_code VARCHAR(50) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    verified BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX idx_mfa_challenges_identity_id ON mfa_challenges(identity_id);
CREATE INDEX idx_mfa_challenges_code ON mfa_challenges(challenge_code);
CREATE INDEX idx_mfa_challenges_expires_at ON mfa_challenges(expires_at);

-- Roles
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    permissions JSONB NOT NULL, -- Array of {resource, action} objects
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_roles_name ON roles(name);

-- User Role Assignments
CREATE TABLE IF NOT EXISTS user_roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identity_id UUID NOT NULL REFERENCES identities(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    assigned_by UUID,
    expires_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_user_roles_identity_id ON user_roles(identity_id);
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);
CREATE INDEX idx_user_roles_expires_at ON user_roles(expires_at);
CREATE UNIQUE INDEX idx_user_roles_unique ON user_roles(identity_id, role_id);

-- Audit Logs
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    identity_id UUID REFERENCES identities(id) ON DELETE SET NULL,
    user_id VARCHAR(255),
    resource VARCHAR(255),
    action VARCHAR(255),
    result VARCHAR(50) NOT NULL, -- SUCCESS, FAILURE, DENIED
    ip_address VARCHAR(45), -- IPv6 compatible
    user_agent TEXT,
    details JSONB,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_audit_logs_identity_id ON audit_logs(identity_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp);
CREATE INDEX idx_audit_logs_result ON audit_logs(result);
CREATE INDEX idx_audit_logs_resource_action ON audit_logs(resource, action);

-- Comments
COMMENT ON TABLE oauth_clients IS 'OAuth 2.0 client registrations';
COMMENT ON TABLE authorization_codes IS 'OAuth 2.0 authorization codes for authorization code flow';
COMMENT ON TABLE access_tokens IS 'OAuth 2.0 access tokens and refresh tokens';
COMMENT ON TABLE mfa_configs IS 'Multi-factor authentication configurations';
COMMENT ON TABLE mfa_challenges IS 'MFA challenge codes for SMS/Email verification';
COMMENT ON TABLE roles IS 'RBAC roles with permissions';
COMMENT ON TABLE user_roles IS 'User-role assignments with optional expiration';
COMMENT ON TABLE audit_logs IS 'Security audit log for compliance and forensics';

