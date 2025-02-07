use reqwest::Url;
use search::SearchRegistryError;
use search::SearchResult;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use crate::AppData;

pub mod resolve;
pub mod search;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum RegistryType {
    Cargo,
    Npm,
    Jsr,
    #[serde(rename = "pypi")]
    PyPI,
}

impl RegistryType {
    pub fn default_url(&self) -> Url {
        match self {
            Self::Cargo => "https://crates.io",
            Self::Npm => "https://registry.npmjs.org/",
            Self::Jsr => "https://api.jsr.io",
            Self::PyPI => "https://pypi.org/simple/",
        }
        .parse()
        .unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Registry {
    registry_type: RegistryType,
    custom_url: Option<Url>,
}

#[tauri::command]
#[specta::specta]
pub async fn search_registry(
    registry: Registry,
    query: String,
    state: State<'_, AppData>,
) -> Result<Vec<SearchResult>, SearchRegistryError> {
    registry.search(&query, Some(state.client.clone())).await
}
