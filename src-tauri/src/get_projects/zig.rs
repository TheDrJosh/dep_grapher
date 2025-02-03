use std::{collections::HashMap, path::Path};

use serde::Deserialize;

pub async fn read_zig_project_info(path: &Path) -> Vec<String> {
    let Ok(build_zig_zon_source) = tokio::fs::read_to_string(path.join("build.zig.zon")).await
    else {
        return vec![];
    };

    let Ok(build_zig_zon) = serde_zon::from_str::<BuildZigZon>(&build_zig_zon_source) else {
        return vec![];
    };

    vec![build_zig_zon.name]
}

#[derive(Debug, Clone, Deserialize)]
struct BuildZigZon {
    name: String,
    version: String,
    #[serde(default)]
    minimum_zig_version: Option<String>,
    dependencies: HashMap<String, BuildZigZonDependency>,
    paths: Vec<String>,
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
        path: String,
    },
}
