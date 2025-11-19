-- Revenue Management Schema
-- Supports real-time charging, usage aggregation, billing cycles, and partner settlements
-- Charging Results table
CREATE TABLE
    IF NOT EXISTS charging_results (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        usage_id UUID NOT NULL UNIQUE,
        rating_id UUID NOT NULL,
        charge_amount_value DECIMAL(15, 2) NOT NULL,
        charge_amount_unit VARCHAR(10) DEFAULT 'USD',
        tax_amount_value DECIMAL(15, 2) NOT NULL DEFAULT 0,
        tax_amount_unit VARCHAR(10) DEFAULT 'USD',
        total_amount_value DECIMAL(15, 2) NOT NULL,
        total_amount_unit VARCHAR(10) DEFAULT 'USD',
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Rating Rules table
CREATE TABLE
    IF NOT EXISTS rating_rules (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        product_offering_id UUID NOT NULL,
        usage_type VARCHAR(100) NOT NULL,
        unit VARCHAR(50) NOT NULL,
        rate_type VARCHAR(20) NOT NULL, -- FLAT, TIERED, VOLUME, TIME_BASED
        base_rate DECIMAL(15, 4) NOT NULL,
        valid_from TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            valid_to TIMESTAMP
        WITH
            TIME ZONE,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (product_offering_id, usage_type, unit, valid_from)
    );

-- Tiered Rates table
CREATE TABLE
    IF NOT EXISTS tiered_rates (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        rating_rule_id UUID NOT NULL REFERENCES rating_rules (id) ON DELETE CASCADE,
        min_quantity DECIMAL(15, 2) NOT NULL,
        max_quantity DECIMAL(15, 2),
        rate DECIMAL(15, 4) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (rating_rule_id, min_quantity)
    );

-- Billing Cycles table
CREATE TABLE
    IF NOT EXISTS billing_cycles (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        customer_id UUID NOT NULL,
        cycle_type VARCHAR(20) NOT NULL, -- MONTHLY, QUARTERLY, ANNUALLY, WEEKLY, CUSTOM
        start_date TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            end_date TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            due_date TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'OPEN', -- OPEN, CLOSED, BILLED, PAID
            bill_id UUID,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Settlement Rules table
CREATE TABLE
    IF NOT EXISTS settlement_rules (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        partner_id UUID NOT NULL,
        product_offering_id UUID,
        revenue_share_percentage DECIMAL(5, 2) NOT NULL, -- Percentage (0-100)
        valid_from TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            valid_to TIMESTAMP
        WITH
            TIME ZONE,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Partner Settlements table
CREATE TABLE
    IF NOT EXISTS partner_settlements (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        partner_id UUID NOT NULL,
        settlement_period_start TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            settlement_period_end TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            total_revenue_value DECIMAL(15, 2) NOT NULL,
            total_revenue_unit VARCHAR(10) DEFAULT 'USD',
            partner_share_value DECIMAL(15, 2) NOT NULL,
            partner_share_unit VARCHAR(10) DEFAULT 'USD',
            platform_share_value DECIMAL(15, 2) NOT NULL,
            platform_share_unit VARCHAR(10) DEFAULT 'USD',
            status VARCHAR(20) NOT NULL DEFAULT 'PENDING', -- PENDING, CALCULATED, APPROVED, PAID, REJECTED
            settlement_date TIMESTAMP
        WITH
            TIME ZONE,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes for charging_results
CREATE INDEX IF NOT EXISTS idx_charging_results_usage_id ON charging_results (usage_id);

CREATE INDEX IF NOT EXISTS idx_charging_results_rating_id ON charging_results (rating_id);

CREATE INDEX IF NOT EXISTS idx_charging_results_created_at ON charging_results (created_at);

-- Indexes for rating_rules
CREATE INDEX IF NOT EXISTS idx_rating_rules_product_offering ON rating_rules (product_offering_id);

CREATE INDEX IF NOT EXISTS idx_rating_rules_usage_type ON rating_rules (usage_type);

CREATE INDEX IF NOT EXISTS idx_rating_rules_valid_dates ON rating_rules (valid_from, valid_to);

-- Indexes for tiered_rates
CREATE INDEX IF NOT EXISTS idx_tiered_rates_rating_rule ON tiered_rates (rating_rule_id);

-- Indexes for billing_cycles
CREATE INDEX IF NOT EXISTS idx_billing_cycles_customer ON billing_cycles (customer_id);

CREATE INDEX IF NOT EXISTS idx_billing_cycles_status ON billing_cycles (status);

CREATE INDEX IF NOT EXISTS idx_billing_cycles_dates ON billing_cycles (start_date, end_date);

CREATE INDEX IF NOT EXISTS idx_billing_cycles_bill_id ON billing_cycles (bill_id);

-- Indexes for settlement_rules
CREATE INDEX IF NOT EXISTS idx_settlement_rules_partner ON settlement_rules (partner_id);

CREATE INDEX IF NOT EXISTS idx_settlement_rules_product ON settlement_rules (product_offering_id);

CREATE INDEX IF NOT EXISTS idx_settlement_rules_valid_dates ON settlement_rules (valid_from, valid_to);

-- Indexes for partner_settlements
CREATE INDEX IF NOT EXISTS idx_partner_settlements_partner ON partner_settlements (partner_id);

CREATE INDEX IF NOT EXISTS idx_partner_settlements_status ON partner_settlements (status);

CREATE INDEX IF NOT EXISTS idx_partner_settlements_period ON partner_settlements (settlement_period_start, settlement_period_end);

-- Comments
COMMENT ON TABLE charging_results IS 'Stores real-time charging results for usage records';

COMMENT ON TABLE rating_rules IS 'Rating rules for different product offerings and usage types';

COMMENT ON TABLE tiered_rates IS 'Tiered rate structures for rating rules';

COMMENT ON TABLE billing_cycles IS 'Billing cycles for customers';

COMMENT ON TABLE settlement_rules IS 'Revenue sharing rules for partners';

COMMENT ON TABLE partner_settlements IS 'Partner settlement records';