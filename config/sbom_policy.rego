package traceguard.sbom

default allow = false

allow {
    input.format == "CycloneDX"
    input.version == "1.4"
    count(input.components) > 0
}

violation[msg] {
    input.format != "CycloneDX"
    msg = "SBOM format must be CycloneDX"
}

violation[msg] {
    input.version != "1.4"
    msg = "SBOM version must be 1.4"
}

violation[msg] {
    count(input.components) == 0
    msg = "SBOM must contain at least one component"
}