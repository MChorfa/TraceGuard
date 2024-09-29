CREATE TABLE provenance_records (
    id SERIAL PRIMARY KEY,
    artifact_id VARCHAR(255) NOT NULL,
    slsa_level INTEGER NOT NULL,
    metadata JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_provenance_records_artifact_id ON provenance_records(artifact_id);