-- Backup and recovery schema
-- Create backup_jobs table
CREATE TABLE
    IF NOT EXISTS backup_jobs (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        tenant_id UUID REFERENCES tenants (id) ON DELETE CASCADE,
        backup_type VARCHAR(50) NOT NULL, -- 'FULL', 'INCREMENTAL', 'DIFFERENTIAL'
        status VARCHAR(50) NOT NULL, -- 'PENDING', 'RUNNING', 'COMPLETED', 'FAILED'
        file_path VARCHAR(500),
        file_size BIGINT,
        started_at TIMESTAMP
        WITH
            TIME ZONE,
            completed_at TIMESTAMP
        WITH
            TIME ZONE,
            error_message TEXT,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Create restore_jobs table
CREATE TABLE
    IF NOT EXISTS restore_jobs (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        tenant_id UUID REFERENCES tenants (id) ON DELETE CASCADE,
        backup_job_id UUID REFERENCES backup_jobs (id) ON DELETE CASCADE,
        status VARCHAR(50) NOT NULL, -- 'PENDING', 'RUNNING', 'COMPLETED', 'FAILED'
        started_at TIMESTAMP
        WITH
            TIME ZONE,
            completed_at TIMESTAMP
        WITH
            TIME ZONE,
            error_message TEXT,
            created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_backup_jobs_tenant_id ON backup_jobs (tenant_id);

CREATE INDEX IF NOT EXISTS idx_backup_jobs_status ON backup_jobs (status);

CREATE INDEX IF NOT EXISTS idx_backup_jobs_created_at ON backup_jobs (created_at);

CREATE INDEX IF NOT EXISTS idx_restore_jobs_tenant_id ON restore_jobs (tenant_id);

CREATE INDEX IF NOT EXISTS idx_restore_jobs_backup_job_id ON restore_jobs (backup_job_id);

CREATE INDEX IF NOT EXISTS idx_restore_jobs_status ON restore_jobs (status);

-- Add comments
COMMENT ON TABLE backup_jobs IS 'Backup job tracking and metadata';

COMMENT ON TABLE restore_jobs IS 'Restore job tracking and metadata';