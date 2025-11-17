-- TMF688 Appointment Management API Schema
-- Appointments table
CREATE TABLE
    IF NOT EXISTS appointments (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(255) NOT NULL,
        description TEXT,
        version VARCHAR(50),
        state VARCHAR(50) NOT NULL DEFAULT 'INITIAL',
        appointment_date TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            duration INTEGER,
            appointment_type VARCHAR(100),
            href VARCHAR(500),
            last_update TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Appointment Related Parties table
CREATE TABLE
    IF NOT EXISTS appointment_related_parties (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        appointment_id UUID NOT NULL REFERENCES appointments (id) ON DELETE CASCADE,
        name VARCHAR(255) NOT NULL,
        role VARCHAR(100) NOT NULL,
        href VARCHAR(500),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Appointment Contact Mediums table
CREATE TABLE
    IF NOT EXISTS appointment_contact_mediums (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        appointment_id UUID NOT NULL REFERENCES appointments (id) ON DELETE CASCADE,
        medium_type VARCHAR(100) NOT NULL,
        value VARCHAR(500) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_appointments_state ON appointments (state);

CREATE INDEX IF NOT EXISTS idx_appointments_appointment_date ON appointments (appointment_date);

CREATE INDEX IF NOT EXISTS idx_appointments_appointment_type ON appointments (appointment_type);

CREATE INDEX IF NOT EXISTS idx_appointment_related_parties_appointment_id ON appointment_related_parties (appointment_id);

CREATE INDEX IF NOT EXISTS idx_appointment_contact_mediums_appointment_id ON appointment_contact_mediums (appointment_id);

-- Comments
COMMENT ON TABLE appointments IS 'TMF688 Appointments - Scheduling technician visits, installations, etc.';

COMMENT ON TABLE appointment_related_parties IS 'TMF688 Appointment Related Parties - Parties related to appointments';

COMMENT ON TABLE appointment_contact_mediums IS 'TMF688 Appointment Contact Mediums - Contact information for appointments';