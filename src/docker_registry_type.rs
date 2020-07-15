#[derive(PartialOrd, PartialEq, Debug)]
pub enum DockerRegistryType {
    Docker,
    IbmCloudRegistry,
}

pub fn parse(s: &str) -> Result<DockerRegistryType, String> {
    match &*s.to_lowercase() {
        "docker" => Result::Ok(DockerRegistryType::Docker),
        "ibmcr" => Result::Ok(DockerRegistryType::IbmCloudRegistry),
        other => Result::Err(format!("Invalid registry type '{}'. Specify 'docker' or 'ibmcr'.", other))
    }
}

#[test]
fn parse_test() {
    assert_eq!(parse("Docker"), Result::Ok(DockerRegistryType::Docker));
    assert_eq!(parse("IBMCR"), Result::Ok(DockerRegistryType::IbmCloudRegistry));
    assert_eq!(parse("skopeo").is_err(), true);
}
