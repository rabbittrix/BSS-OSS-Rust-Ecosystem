-- TMF666 Account Management, TMF676 Payment Management, TMF651 Agreement Management

CREATE TABLE IF NOT EXISTS billing_accounts (
    id UUID PRIMARY KEY,
    href TEXT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    state VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    account_type VARCHAR(100),
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS party_accounts (
    id UUID PRIMARY KEY,
    href TEXT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    state VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    account_type VARCHAR(100),
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY,
    href TEXT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    amount JSONB,
    payment_date TIMESTAMPTZ,
    billing_account_id UUID,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS refunds (
    id UUID PRIMARY KEY,
    href TEXT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    amount JSONB,
    refund_date TIMESTAMPTZ,
    payment_id UUID,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agreements (
    id UUID PRIMARY KEY,
    href TEXT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    status VARCHAR(50) NOT NULL DEFAULT 'IN_PROCESS',
    agreement_type VARCHAR(100),
    period_start TIMESTAMPTZ,
    period_end TIMESTAMPTZ,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_billing_accounts_state ON billing_accounts (state);
CREATE INDEX IF NOT EXISTS idx_party_accounts_state ON party_accounts (state);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments (status);
CREATE INDEX IF NOT EXISTS idx_payments_billing_account ON payments (billing_account_id);
CREATE INDEX IF NOT EXISTS idx_refunds_status ON refunds (status);
CREATE INDEX IF NOT EXISTS idx_refunds_payment ON refunds (payment_id);
CREATE INDEX IF NOT EXISTS idx_agreements_status ON agreements (status);

COMMENT ON TABLE billing_accounts IS 'TMF666 Billing Account';
COMMENT ON TABLE party_accounts IS 'TMF666 Party Account';
COMMENT ON TABLE payments IS 'TMF676 Payment';
COMMENT ON TABLE refunds IS 'TMF676 Refund';
COMMENT ON TABLE agreements IS 'TMF651 Agreement';
