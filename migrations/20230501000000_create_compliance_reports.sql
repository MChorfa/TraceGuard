CREATE TABLE compliance_reports (
    id SERIAL PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    report_type VARCHAR(50) NOT NULL,
    content JSONB NOT NULL,
    generated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_compliance_reports_tenant_id ON compliance_reports(tenant_id);