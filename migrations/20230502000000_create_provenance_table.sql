CREATE TABLE provenance (
    id SERIAL PRIMARY KEY,
    artifact_id VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL,
    metadata JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_provenance_artifact_id ON provenance(artifact_id);