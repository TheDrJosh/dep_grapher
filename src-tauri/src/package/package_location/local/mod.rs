use std::{io::ErrorKind, path::PathBuf};

use deno::resolve_deno_package_info;
use nodejs::resolve_nodejs_package_info;
use python::resolve_python_package_info;
use rust::resolve_rust_package_info;
use serde::{Deserialize, Serialize};
use specta::Type;
use zig::resolve_zig_package_info;

use super::{Package, PackageLocation};

mod deno;
mod nodejs;
mod python;
mod rust;
mod zig;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LocalPackageLocation {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Type, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum InvalidPathError {
    #[error("path not found.")]
    NotFound,
    #[error("path not a directory.")]
    NotADirectory,
    #[error("path conatins invalid characters")]
    InvalidName,
    #[error("path not absolute")]
    NotAbsolute,
    #[error("unknown io error: {0}")]
    Unknown(
        #[from]
        #[serde(skip)]
        std::io::Error,
    ),
}

#[derive(Debug, Serialize, Type, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum LocalResolveError {
    #[error("no package in path")]
    NoPackagesInPath,
    #[error("{0}")]
    InvalidPath(#[from] InvalidPathError),
}

impl LocalPackageLocation {
    pub async fn is_valid(&self) -> Result<(), InvalidPathError> {
        if self.path.is_relative() {
            return Err(InvalidPathError::NotAbsolute);
        }

        let metadata = tokio::fs::metadata(&self.path)
            .await
            .map_err(|err| match err.kind() {
                ErrorKind::NotFound => InvalidPathError::NotFound,
                ErrorKind::InvalidFilename => InvalidPathError::InvalidName,
                _ => InvalidPathError::Unknown(err),
            })?;

        if !metadata.is_dir() {
            return Err(InvalidPathError::NotADirectory);
        }

        Ok(())
    }

    pub async fn resolve(&self) -> Result<Vec<Package>, LocalResolveError> {
        self.is_valid().await?;

        let mut packages = vec![];

        let location = PackageLocation::Local(self.clone());

        packages.append(&mut resolve_rust_package_info(&self.path, &location).await);

        packages.append(&mut resolve_nodejs_package_info(&self.path, &location).await);

        if let Some(package) = resolve_python_package_info(&self.path, &location).await {
            packages.push(package);
        }

        packages.append(&mut resolve_deno_package_info(&self.path, &location).await);

        if let Some(package) = resolve_zig_package_info(&self.path, &location).await {
            packages.push(package);
        }

        if packages.len() == 0 {
            return Err(LocalResolveError::NoPackagesInPath);
        }

        Ok(packages)
    }
}

// pub async fn get_projects_in_dir(path: PathBuf) -> Result<Vec<Project>, ProjectsInDirError> {
//     //TODO - Should be an unwrap - broken on client side
//     is_path_valid(path.clone())
//         .await
//         .map_err(|_| ProjectsInDirError::NotAProjectPath)?;

//     let mut projects = vec![];

//     let start = Instant::now();

//     debug!("rust start");

//     projects.append(
//         &mut read_rust_project_info(&path)
//             .await
//             .into_iter()
//             .map(|name| Project {
//                 name,
//                 project_type: ProjectType::Rust,
//             })
//             .collect(),
//     );

//     debug!("rust end / node js start");

//     projects.append(
//         &mut read_nodejs_project_info(&path)
//             .await
//             .into_iter()
//             .map(|name| Project {
//                 name,
//                 project_type: ProjectType::NodeJS,
//             })
//             .collect(),
//     );

//     debug!("node js end / python start");

//     //TODO - Should Update
//     projects.append(
//         &mut read_python_project_info(&path)
//             .await
//             .into_iter()
//             .map(|name| Project {
//                 name,
//                 project_type: ProjectType::Python,
//             })
//             .collect(),
//     );

//     debug!("python end / deno start");

//     projects.append(
//         &mut read_deno_project_info(&path)
//             .await
//             .into_iter()
//             .map(|name| Project {
//                 name,
//                 project_type: ProjectType::Deno,
//             })
//             .collect(),
//     );

//     debug!("deno end / zig start");

//     //TODO - Should Update
//     projects.append(
//         &mut read_zig_project_info(&path)
//             .await
//             .into_iter()
//             .map(|name| Project {
//                 name,
//                 project_type: ProjectType::Zig,
//             })
//             .collect(),
//     );

//     debug!("zig end");

//     let end = Instant::now();

//     debug!("e time: {:?}", end - start);

//     if projects.len() == 0 {
//         return Err(ProjectsInDirError::NotAProjectPath);
//     }

//     Ok(projects)
// }
