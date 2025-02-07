use std::path::PathBuf;

use git::{GitPackageLocation, GitResolveError};
use local::{InvalidPathError, LocalPackageLocation, LocalResolveError};
use registry::RegistryPackageLocation;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::Url;

use crate::registry::resolve::RegistryResolveError;

use super::Package;

pub mod git;
pub mod local;
pub mod registry;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum PackageLocation {
    Local(LocalPackageLocation),
    Git(GitPackageLocation),
    Registry(RegistryPackageLocation),
    Url(Url),
    Unknown {
        name: String,
        description: String,
    }
}

#[derive(Debug, Serialize, Type, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum ResolveError {
    #[error("failed to resolve local package: {0}")]
    Local(#[from] LocalResolveError),
    #[error("failed to resolve git package: {0}")]
    Git(#[from] GitResolveError),
    #[error("failed to resolve registry package: {0}")]
    Registry(#[from] RegistryResolveError),
}

impl PackageLocation {
    pub async fn resolve(&self) -> Result<Vec<Package>, ResolveError> {
        Ok(match self {
            PackageLocation::Local(local_package) => local_package.resolve().await?,
            PackageLocation::Git(git_package) => git_package.resolve().await?,
            PackageLocation::Registry(registry_package) => vec![registry_package.resolve().await?],
            PackageLocation::Url(url) => todo!(),
            PackageLocation::Unknown { name, description } => todo!(),
        })
    }
}

#[tauri::command]
#[specta::specta]
pub async fn resolve_package(location: PackageLocation) -> Result<Vec<Package>, ResolveError> {
    location.resolve().await
}

#[tauri::command]
#[specta::specta]
pub async fn is_path_valid(path: PathBuf) -> Result<(), InvalidPathError> {
    LocalPackageLocation { path }.is_valid().await
}
