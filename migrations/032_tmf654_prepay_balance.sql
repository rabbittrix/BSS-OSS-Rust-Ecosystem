-- TMF654 Prepay Balance Management

CREATE TABLE IF NOT EXISTS prepay_balances (
    id UUID PRIMARY KEY,
    href TEXT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50),
    balance_type VARCHAR(50) NOT NULL DEFAULT 'MONETARY',
    remaining_value JSONB NOT NULL DEFAULT '{"value":0,"unit":"EUR"}',
    party_id UUID,
    valid_for_end TIMESTAMPTZ,
    last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_prepay_balances_party ON prepay_balances (party_id);
CREATE INDEX IF NOT EXISTS idx_prepay_balances_type ON prepay_balances (balance_type);

COMMENT ON TABLE prepay_balances IS 'TMF654 Prepay Balance';
