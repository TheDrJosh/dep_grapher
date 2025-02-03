use std::path::PathBuf;

use deno::read_deno_project_info;
use nodejs::read_nodejs_project_info;
use python::read_python_project_info;
use rust::read_rust_project_info;
use serde::Serialize;
use zig::read_zig_project_info;

use crate::is_path_valid;

mod deno;
mod nodejs;
mod python;
mod rust;
mod zig;

#[derive(Debug, Serialize)]
pub enum ProjectsInDirError {}

#[derive(Debug, Serialize)]
pub enum ProjectType {
    Rust,
    NodeJS,
    Deno,
    Python,
    Zig,
}

#[derive(Debug, Serialize)]
pub struct Project {
    name: String,
    project_type: ProjectType,
}

#[tauri::command]
pub async fn get_projects_in_dir(path: PathBuf) -> Result<Vec<Project>, ProjectsInDirError> {
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

    projects.append(
        &mut read_nodejs_project_info(&path)
            .await
            .into_iter()
            .map(|name| Project {
                name,
                project_type: ProjectType::NodeJS,
            })
            .collect(),
    );

    projects.append(
        &mut read_python_project_info(&path)
            .await
            .into_iter()
            .map(|name| Project {
                name,
                project_type: ProjectType::Python,
            })
            .collect(),
    );

    projects.append(
        &mut read_deno_project_info(&path)
            .await
            .into_iter()
            .map(|name| Project {
                name,
                project_type: ProjectType::Deno,
            })
            .collect(),
    );

    projects.append(
        &mut read_zig_project_info(&path)
            .await
            .into_iter()
            .map(|name| Project {
                name,
                project_type: ProjectType::Zig,
            })
            .collect(),
    );

    todo!()
}
