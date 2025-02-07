use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::Url;

use super::Package;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct GitPackageLocation {
    pub git_type: GitPackageType,
    pub commit: GitCommit,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum GitCommit {
    Commit(String),
    Tag(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum GitPackageType {
    Local(PathBuf),
    Remote(Url),
}

#[derive(Debug, Clone, Serialize, Type, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum GitResolveError {}

impl GitPackageLocation {
    pub async fn resolve(&self) -> Result<Vec<Package>, GitResolveError> {
        todo!()
    }
}
