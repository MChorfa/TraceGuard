use traceguard::sbom::sbom_parser;
use traceguard::provenance::provenance_api;
use traceguard::compliance::oscal;
use traceguard::security::encryption;
use traceguard::lifecycle::lifecycle_manager;
use traceguard::plugins::plugin_manager;

#[tokio::test]
async fn test_sbom_parsing_and_provenance() {
    let sbom_data = r#"
    {
        "bomFormat": "CycloneDX",
        "specVersion": "1.4",
        "version": 1,
        "components": [
            {
                "type": "library",
                "name": "example-lib",
                "version": "1.0.0"
            }
        ]
    }
    "#;

    let sbom = sbom_parser::parse_sbom_from_string(sbom_data).unwrap();
    assert_eq!(sbom.format, "CycloneDX");
    assert_eq!(sbom.components.len(), 1);

    let provenance_record = provenance_api::record_provenance("example-artifact", 2, None).await.unwrap();
    assert_eq!(provenance_record.slsa_level, 2);
}

#[test]
fn test_oscal_report_generation() {
    let system_name = "TestSystem";
    let components = vec![
        oscal::OSCALComponent {
            uuid: uuid::Uuid::new_v4(),
            type_: "software".to_string(),
            title: "TestComponent".to_string(),
            description: "A test component".to_string(),
            props: vec![],
        },
    ];

    let report = oscal::generate_oscal_report(system_name, components);
    assert_eq!(report.title, "OSCAL Report for TestSystem");
    assert_eq!(report.components.len(), 1);

    let json = oscal::export_oscal_json(&report).unwrap();
    assert!(json.contains("TestSystem"));
    assert!(json.contains("TestComponent"));
}

#[tokio::test]
async fn test_encryption() {
    let encryption_manager = encryption::EncryptionManager::new();
    let tenant_id = uuid::Uuid::new_v4();
    encryption_manager.generate_tenant_key(tenant_id).unwrap();

    let data = b"Sensitive data";
    let encrypted = encryption_manager.encrypt(tenant_id, data).unwrap();
    let decrypted = encryption_manager.decrypt(tenant_id, &encrypted).unwrap();

    assert_eq!(data.to_vec(), decrypted);
}

#[tokio::test]
async fn test_lifecycle_management() {
    let lifecycle_manager = lifecycle_manager::LifecycleManager::new().await.unwrap();
    let policy = lifecycle_manager::LifecyclePolicy {
        id: uuid::Uuid::new_v4(),
        tenant_id: uuid::Uuid::new_v4(),
        artifact_type: "SBOM".to_string(),
        retention_period: chrono::Duration::days(30),
        archive_after: Some(chrono::Duration::days(60)),
        delete_after: Some(chrono::Duration::days(90)),
    };

    lifecycle_manager.apply_policy(&policy).await.unwrap();
    let applied_policy = lifecycle_manager.get_policy(policy.id).await.unwrap();
    assert_eq!(policy.retention_period, applied_policy.retention_period);
}

#[tokio::test]
async fn test_plugin_execution() {
    let plugin_manager = plugin_manager::PluginManager::new();
    let result = plugin_manager.execute_plugin("guac", &serde_json::json!({
        "action": "analyze_dependencies",
        "artifact_id": "example-artifact"
    })).await.unwrap();

    assert!(result.contains("analysis_complete"));
}
