use std::{path::PathBuf, time::{Duration, Instant}};

use deno::read_deno_project_info;
use log::debug;
use nodejs::read_nodejs_project_info;
use python::read_python_project_info;
use rust::read_rust_project_info;
use serde::Serialize;
use specta::Type;
use zig::read_zig_project_info;

use crate::is_path_valid;

mod deno;
mod nodejs;
mod python;
mod rust;
mod zig;

#[derive(Debug, Serialize, Type)]
pub enum ProjectsInDirError {
    NotAProjectPath,
}

#[derive(Debug, Serialize, Type)]
pub enum ProjectType {
    Rust,
    NodeJS,
    Deno,
    Python,
    Zig,
}

#[derive(Debug, Serialize, Type)]
pub struct Project {
    name: String,
    project_type: ProjectType,
}

#[tauri::command]
#[specta::specta]
pub async fn get_projects_in_dir(path: PathBuf) -> Result<Vec<Project>, ProjectsInDirError> {
    //TODO - Should be an unwrap - broken on client side
    is_path_valid(path.clone())
        .await
        .map_err(|_| ProjectsInDirError::NotAProjectPath)?;

    let mut projects = vec![];

    let start = Instant::now();

    debug!("rust start");

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

    debug!("rust end / node js start");
    
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

    debug!("node js end / python start");


    //TODO - Should Update
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

    debug!("python end / deno start");


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

    debug!("deno end / zig start");

    //TODO - Should Update
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

    debug!("zig end");

    let end = Instant::now();

    debug!("e time: {:?}", end - start);


    if projects.len() == 0 {
        return Err(ProjectsInDirError::NotAProjectPath);
    }

    Ok(projects)
}
