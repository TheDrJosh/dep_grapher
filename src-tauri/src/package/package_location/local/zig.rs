use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use tauri::{path::SafePathBuf, Url};

use crate::package::{package_location::PackageLocation, Language, Package};

pub async fn resolve_zig_package_info(path: &Path, location: &PackageLocation) -> Option<Package> {
    let build_zig_zon_source = tokio::fs::read_to_string(path.join("build.zig.zon"))
        .await
        .ok()?;

    let build_zig_zon = serde_zon::from_str::<BuildZigZon>(&build_zig_zon_source).ok()?;

    Some(Package {
        location: location.clone(),
        name: build_zig_zon.name,
        version: build_zig_zon.version,
        description: None,
        authors: vec![],
        language: Language::Zig,
        dependencies: build_zig_zon
            .dependencies
            .into_iter()
            .map(|(_, dep)| match dep {
                BuildZigZonDependency::Url {
                    url,
                    hash: _,
                    lazy: _,
                } => {
                    PackageLocation::Url(Url::parse(&url).unwrap()) // TODO: handle git urls
                }
                BuildZigZonDependency::Path { path } => match SafePathBuf::new(path.clone()) {
                    Ok(_) => PackageLocation::Local(super::LocalPackageLocation { path }),
                    Err(err) => PackageLocation::Unknown {
                        name: path.to_string_lossy().to_string(),
                        description: err.to_string(),
                    },
                },
            })
            .collect(),
    })
}

#[derive(Debug, Clone, Deserialize)]
struct BuildZigZon {
    name: String,
    version: String,
    #[serde(default)]
    minimum_zig_version: Option<String>,
    dependencies: HashMap<String, BuildZigZonDependency>,
    paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum BuildZigZonDependency {
    Url {
        url: String,
        hash: String,
        lazy: bool,
    },
    Path {
        path: PathBuf,
    },
}
