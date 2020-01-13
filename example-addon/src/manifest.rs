use semver::Version;
use stremio_core::types::addons::*;
use stremio_addon_sdk::scaffold::Scaffold;

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
        id_prefixes: Some(vec!["tt".into()]),
        description: Some("Rust addon test".into()),
        ..Scaffold::default_manifest()
    }
}
