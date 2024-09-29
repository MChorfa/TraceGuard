use traceguard::sbom::sbom_parser::{parse_sbom, SBOM, Component};

#[test]
fn test_parse_cyclonedx_sbom() {
    let cyclonedx_sbom = r#"
    {
        "bomFormat": "CycloneDX",
        "specVersion": "1.4",
        "version": 1,
        "components": [
            {
                "type": "library",
                "name": "example-lib",
                "version": "1.0.0",
                "purl": "pkg:generic/example-lib@1.0.0"
            }
        ]
    }
    "#;

    let result = parse_sbom(cyclonedx_sbom);
    assert!(result.is_ok());

    let sbom = result.unwrap();
    assert_eq!(sbom.format, "CycloneDX");
    assert_eq!(sbom.version, "1.4");
    assert_eq!(sbom.components.len(), 1);

    let component = &sbom.components[0];
    assert_eq!(component.name, "example-lib");
    assert_eq!(component.version, "1.0.0");
    assert_eq!(component.purl, Some("pkg:generic/example-lib@1.0.0".to_string()));
}