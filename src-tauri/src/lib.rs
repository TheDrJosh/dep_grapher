#![feature(io_error_more)]

use std::{
    collections::HashMap,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_path_valid, get_projects_in_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize)]
enum InvalidPath {
    NotFound,
    NotADirectory,
    InvalidName,
    NotAbsolute,
    Unknown,
}

#[tauri::command]
async fn is_path_valid(path: PathBuf) -> Result<(), InvalidPath> {
    if path.is_relative() {
        return Err(InvalidPath::NotAbsolute);
    }

    let metadata = tokio::fs::metadata(path)
        .await
        .map_err(|err| match err.kind() {
            ErrorKind::NotFound => InvalidPath::NotFound,
            ErrorKind::InvalidFilename => InvalidPath::InvalidName,
            _ => InvalidPath::Unknown,
        })?;

    if !metadata.is_dir() {
        return Err(InvalidPath::NotADirectory);
    }

    Ok(())
}

#[derive(Debug, Serialize)]
enum ProjectsInDirError {}

#[derive(Debug, Serialize)]
enum ProjectType {
    Rust,
    NodeJS,
    Deno,
    Python,
    Zig,
}

#[derive(Debug, Serialize)]
struct Project {
    name: String,
    project_type: ProjectType,
}

#[tauri::command]
async fn get_projects_in_dir(path: PathBuf) -> Result<Vec<Project>, ProjectsInDirError> {
    is_path_valid(path.clone()).await.unwrap();

    let mut projects = vec![];

    projects.append(
        &mut read_rust_project_info(&path)
            .await
            .into_iter()
            .map(|name| Project {
                name,
                project_type: ProjectType::Rust,
            })
            .collect(),
    );

    todo!()
}

async fn read_rust_project_info(path: &Path) -> Vec<String> {
    let mut names = vec![];

    let Ok(manifest) = cargo_toml::Manifest::from_path(path.join("Cargo.toml")) else {
        return names;
    };

    // manifest.complete_from_path(path).ok()?;

    if let Some(package) = manifest.package {
        names.push(package.name);
    }

    if let Some(workspace) = manifest.workspace {
        let exclude_matchers = workspace
            .exclude
            .iter()
            .filter_map(|exclude| globmatch::Builder::new(&exclude).build(path).ok())
            .collect::<Vec<_>>();

        for member in workspace.members {
            let Ok(member_matcher) = globmatch::Builder::new(&member).build(path) else {
                //TODO - add log
                continue;
            };

            'member_loop: for member_path in member_matcher.into_iter() {
                let Ok(member_path) = member_path else {
                    //TODO - add log
                    continue;
                };

                for exclude_matcher in &exclude_matchers {
                    if exclude_matcher.is_match(member_path.clone()) {
                        continue 'member_loop;
                    }
                }

                names.append(&mut Box::pin(read_rust_project_info(&member_path)).await);
            }
        }
    }

    todo!()
}

async fn read_nodejs_project_info(path: &Path) -> Vec<String> {
    todo!()
}

async fn read_deno_project_info(path: &Path) -> Vec<String> {
    todo!()
}

async fn read_python_project_info(path: &Path) -> Vec<String> {
    todo!()
}

async fn read_zig_project_info(path: &Path) -> Vec<String> {
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
