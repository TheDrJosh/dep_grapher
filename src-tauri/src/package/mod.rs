use package_location::PackageLocation;
use serde::{Deserialize, Serialize};
use specta::Type;

pub mod package_location;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Package {
    pub location: PackageLocation,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub language: Language,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Dependency {
    location: PackageLocation,
    name: String,
    stage: Stage,
    required_features: Vec<String>,
    with_features: Vec<String>,
    platform_requirements: PlatformRequirements,
    optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum Stage {
    Default,
    Dev,
    Build,
    Custom(String)
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
enum PlatformRequirements {
    Whitelist(Vec<String>),
    Blacklist(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Rust,
    NodeJS,
    Deno,
    Python,
    Zig,
}
