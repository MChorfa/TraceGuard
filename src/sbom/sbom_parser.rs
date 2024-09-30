use serde::{Deserialize, Serialize};
use thiserror::Error;
use spdx_rs::models::SPDX;
use swid::Tag;
use log::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct SBOM {
    pub format: String,
    pub version: String,
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub version: String,
    pub purl: Option<String>,
}

#[derive(Error, Debug)]
pub enum SBOMError {
    #[error("Failed to parse SBOM: {0}")]
    ParseError(String),
    #[error("Unsupported SBOM format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

pub fn parse_sbom(contents: &str) -> Result<SBOM, SBOMError> {
    info!("Parsing SBOM");
    if contents.contains("CycloneDX") {
        parse_cyclonedx(contents)
    } else if contents.contains("SPDX") {
        parse_spdx(contents)
    } else if contents.contains("SWID") {
        parse_swid(contents)
    } else {
        error!("Unknown SBOM format");
        Err(SBOMError::UnsupportedFormat("Unknown format".to_string()))
    }
}

fn parse_cyclonedx(contents: &str) -> Result<SBOM, SBOMError> {
    info!("Parsing CycloneDX SBOM");
    serde_json::from_str(contents)
        .map_err(|e| {
            error!("Failed to parse CycloneDX SBOM: {}", e);
            SBOMError::ParseError(e.to_string())
        })
        .and_then(|cyclonedx: cyclonedx_bom::models::Bom| {
            Ok(SBOM {
                format: "CycloneDX".to_string(),
                version: cyclonedx.version.to_string(),
                components: cyclonedx.components.into_iter()
                    .map(|c| Component {
                        name: c.name,
                        version: c.version.unwrap_or_default(),
                        purl: c.purl,
                    })
                    .collect(),
            })
        })
}

fn parse_spdx(contents: &str) -> Result<SBOM, SBOMError> {
    info!("Parsing SPDX SBOM");
    serde_json::from_str::<SPDX>(contents)
        .map_err(|e| {
            error!("Failed to parse SPDX SBOM: {}", e);
            SBOMError::ParseError(e.to_string())
        })
        .and_then(|spdx| {
            Ok(SBOM {
                format: "SPDX".to_string(),
                version: spdx.document_creation_information.spdx_version,
                components: spdx.package_information.into_iter()
                    .map(|p| Component {
                        name: p.package_name,
                        version: p.package_version.unwrap_or_default(),
                        purl: p.external_reference.into_iter()
                            .find(|r| r.reference_type == "purl")
                            .map(|r| r.reference_locator),
                    })
                    .collect(),
            })
        })
}

fn parse_swid(contents: &str) -> Result<SBOM, SBOMError> {
    info!("Parsing SWID SBOM");
    let tag: Tag = quick_xml::de::from_str(contents)
        .map_err(|e| {
            error!("Failed to parse SWID SBOM: {}", e);
            SBOMError::ParseError(e.to_string())
        })?;

    Ok(SBOM {
        format: "SWID".to_string(),
        version: tag.version.unwrap_or_default(),
        components: tag.software_identity.components.into_iter()
            .map(|c| Component {
                name: c.name,
                version: c.version.unwrap_or_default(),
                purl: None, // SWID doesn't have a direct PURL equivalent
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cyclonedx() {
        let cyclonedx_content = r#"
        {
            "bomFormat": "CycloneDX",
            "specVersion": "1.4",
            "version": 1,
            "components": [
                {
                    "name": "component1",
                    "version": "1.0.0",
                    "purl": "pkg:generic/component1@1.0.0"
                }
            ]
        }
        "#;

        let result = parse_sbom(cyclonedx_content);
        assert!(result.is_ok());
        let sbom = result.unwrap();
        assert_eq!(sbom.format, "CycloneDX");
        assert_eq!(sbom.components.len(), 1);
        assert_eq!(sbom.components[0].name, "component1");
    }

    #[test]
    fn test_parse_spdx() {
        let spdx_content = r#"
        {
            "spdxVersion": "SPDX-2.2",
            "dataLicense": "CC0-1.0",
            "SPDXID": "SPDXRef-DOCUMENT",
            "name": "example",
            "documentNamespace": "http://spdx.org/spdxdocs/example-v1.0",
            "creationInfo": {
                "created": "2022-01-01T00:00:00Z",
                "creators": ["Tool: example-tool-1.0"]
            },
            "packages": [
                {
                    "name": "package1",
                    "SPDXID": "SPDXRef-Package-1",
                    "versionInfo": "1.0.0",
                    "downloadLocation": "NOASSERTION",
                    "filesAnalyzed": false,
                    "licenseConcluded": "NOASSERTION",
                    "licenseDeclared": "NOASSERTION",
                    "copyrightText": "NOASSERTION",
                    "externalRefs": [
                        {
                            "referenceCategory": "PACKAGE-MANAGER",
                            "referenceType": "purl",
                            "referenceLocator": "pkg:generic/package1@1.0.0"
                        }
                    ]
                }
            ]
        }
        "#;

        let result = parse_sbom(spdx_content);
        assert!(result.is_ok());
        let sbom = result.unwrap();
        assert_eq!(sbom.format, "SPDX");
        assert_eq!(sbom.components.len(), 1);
        assert_eq!(sbom.components[0].name, "package1");
    }

    #[test]
    fn test_parse_swid() {
        let swid_content = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <SoftwareIdentity
          name="Example Software"
          tagId="example-software-1.0.0"
          version="1.0.0"
          versionScheme="semver"
          xmlns="http://standards.iso.org/iso/19770/-2/2015/schema.xsd">
          <Entity
            name="Example Corp"
            regid="example.com"
            role="softwareCreator"/>
          <Link
            rel="license"
            href="https://example.com/license"/>
        </SoftwareIdentity>
        "#;

        let result = parse_sbom(swid_content);
        assert!(result.is_ok());
        let sbom = result.unwrap();
        assert_eq!(sbom.format, "SWID");
        assert_eq!(sbom.version, "1.0.0");
    }
}