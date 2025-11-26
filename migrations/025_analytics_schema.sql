-- Analytics and reporting schema
-- Create analytics_reports table
CREATE TABLE
    IF NOT EXISTS analytics_reports (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        report_type VARCHAR(50) NOT NULL,
        tenant_id UUID REFERENCES tenants (id) ON DELETE CASCADE,
        time_range_start TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            time_range_end TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            data JSONB NOT NULL,
            generated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_analytics_reports_tenant_id ON analytics_reports (tenant_id);

CREATE INDEX IF NOT EXISTS idx_analytics_reports_type ON analytics_reports (report_type);

CREATE INDEX IF NOT EXISTS idx_analytics_reports_generated_at ON analytics_reports (generated_at);

CREATE INDEX IF NOT EXISTS idx_analytics_reports_time_range ON analytics_reports (time_range_start, time_range_end);

-- Add comments
COMMENT ON TABLE analytics_reports IS 'Stored analytics reports for historical analysis';

COMMENT ON COLUMN analytics_reports.report_type IS 'Type of report: Sales, Revenue, Usage, Orders, Customers, Products, Custom';

COMMENT ON COLUMN analytics_reports.data IS 'Report data in JSON format';