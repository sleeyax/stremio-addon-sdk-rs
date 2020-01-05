use semver::Version;
use stremio_core::types::addons::*;

pub fn get_manifest() -> Manifest {
    Manifest {
        id: "org.test".into(),
        name: "Rust Example Addon".into(),
        version: Version::new(1, 0, 0),
        resources: vec![
            ManifestResource::Short("catalog".into()),
            ManifestResource::Short("stream".into())
        ],
        types: vec!["movie".into()],
        catalogs: vec![
            ManifestCatalog {
                type_name: "others".into(),
                id: "bbbcatalog".into(),
                name: Some("Rust test".into()),
                extra: ManifestExtra::default()
            }
        ],
        contact_email: None,
        background: None,
        logo: None,
        id_prefixes: Some(vec!["tt".into()]),
        description: Some("Rust addon test".into()),
        addon_catalogs: vec![],
        behavior_hints:  Default::default() // serde_json::map::Map::new()
    }
}
