use stremio_core::types::addons::{Manifest};
use semver::Version;
use serde_json;

pub struct Scaffold;
impl Scaffold {
    pub fn default_manifest() -> Manifest {
        Manifest {
            id: String::default(),
            name: String::default(),
            version: Version::new(0, 0, 1),
            resources: Vec::default(),
            types: Vec::default(),
            catalogs: Vec::default(),
            contact_email: Option::default(),
            background: Option::default(),
            logo: Option::default(),
            id_prefixes: Option::default(),
            description: Option::default(),
            addon_catalogs: Vec::default(),
            behavior_hints: serde_json::map::Map::default() // Default::default()
        }
    }
}