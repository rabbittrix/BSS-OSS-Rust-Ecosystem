-- TMF642 Alarm Management Schema
CREATE TABLE
    IF NOT EXISTS alarms (
        id UUID PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'RAISED',
        severity VARCHAR(50) NOT NULL,
        alarm_type VARCHAR(100) NOT NULL,
        source_resource_id UUID,
        raised_time TIMESTAMP
        WITH
            TIME ZONE,
            acknowledged_time TIMESTAMP
        WITH
            TIME ZONE,
            cleared_time TIMESTAMP
        WITH
            TIME ZONE,
            alarm_details TEXT,
            href VARCHAR(500),
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

CREATE INDEX IF NOT EXISTS idx_alarms_state ON alarms (state);

CREATE INDEX IF NOT EXISTS idx_alarms_severity ON alarms (severity);

CREATE INDEX IF NOT EXISTS idx_alarms_type ON alarms (alarm_type);

CREATE INDEX IF NOT EXISTS idx_alarms_raised_time ON alarms (raised_time);

CREATE INDEX IF NOT EXISTS idx_alarms_source_resource ON alarms (source_resource_id);