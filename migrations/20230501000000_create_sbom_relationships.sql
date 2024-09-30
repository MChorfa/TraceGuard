-- Create sbom_relationships table
CREATE TABLE IF NOT EXISTS sbom_relationships (
    id SERIAL PRIMARY KEY,
    source_sbom_id UUID NOT NULL,
    target_sbom_id UUID NOT NULL,
    relationship_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_sbom_id) REFERENCES sboms(id) ON DELETE CASCADE,
    FOREIGN KEY (target_sbom_id) REFERENCES sboms(id) ON DELETE CASCADE
);

-- Create index for faster querying
CREATE INDEX idx_sbom_relationships_source ON sbom_relationships(source_sbom_id);
CREATE INDEX idx_sbom_relationships_target ON sbom_relationships(target_sbom_id);