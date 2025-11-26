-- IoT Device Management schema
-- Create iot_devices table
CREATE TABLE
    IF NOT EXISTS iot_devices (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        device_type VARCHAR(50) NOT NULL,
        manufacturer VARCHAR(255) NOT NULL,
        model VARCHAR(255) NOT NULL,
        serial_number VARCHAR(255) NOT NULL UNIQUE,
        firmware_version VARCHAR(100),
        hardware_version VARCHAR(100),
        mac_address VARCHAR(17),
        ip_address VARCHAR(45),
        status VARCHAR(50) NOT NULL DEFAULT 'REGISTERED',
        location JSONB,
        capabilities JSONB DEFAULT '[]'::jsonb,
        configuration JSONB,
        tenant_id UUID REFERENCES tenants (id) ON DELETE CASCADE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            last_update TIMESTAMP
        WITH
            TIME ZONE,
            last_seen TIMESTAMP
        WITH
            TIME ZONE
    );

-- Create iot_telemetry table
CREATE TABLE
    IF NOT EXISTS iot_telemetry (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        device_id UUID NOT NULL REFERENCES iot_devices (id) ON DELETE CASCADE,
        timestamp TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
            metrics JSONB NOT NULL,
            tags JSONB,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_iot_devices_tenant_id ON iot_devices (tenant_id);
CREATE INDEX IF NOT EXISTS idx_iot_devices_status ON iot_devices (status);
CREATE INDEX IF NOT EXISTS idx_iot_devices_device_type ON iot_devices (device_type);
CREATE INDEX IF NOT EXISTS idx_iot_devices_serial_number ON iot_devices (serial_number);
CREATE INDEX IF NOT EXISTS idx_iot_devices_last_seen ON iot_devices (last_seen);

CREATE INDEX IF NOT EXISTS idx_iot_telemetry_device_id ON iot_telemetry (device_id);
CREATE INDEX IF NOT EXISTS idx_iot_telemetry_timestamp ON iot_telemetry (timestamp);
CREATE INDEX IF NOT EXISTS idx_iot_telemetry_device_timestamp ON iot_telemetry (device_id, timestamp DESC);

-- Add comments
COMMENT ON TABLE iot_devices IS 'IoT device registry and management';
COMMENT ON COLUMN iot_devices.device_type IS 'Device type: SENSOR, ACTUATOR, GATEWAY, CONTROLLER, SMART_METER, ROUTER, SWITCH, OTHER';
COMMENT ON COLUMN iot_devices.status IS 'Device status: REGISTERED, PROVISIONED, ACTIVE, INACTIVE, OFFLINE, MAINTENANCE, DECOMMISSIONED';
COMMENT ON TABLE iot_telemetry IS 'IoT device telemetry data storage';

