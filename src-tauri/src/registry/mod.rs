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
pub enum Registry {
    Cargo(Option<Url>),
    Npm(Option<Url>),
    Jsr(Option<Url>),
    PyPI(Option<Url>),
}

impl Registry {
    pub fn default_url(&self) -> Url {
        match self {
            Registry::Cargo(_) => "https://crates.io",
            Registry::Npm(_) => "https://registry.npmjs.org/",
            Registry::Jsr(_) => "https://api.jsr.io",
            Registry::PyPI(_) => "https://pypi.org/simple/",
        }
        .parse()
        .unwrap()
    }

    pub fn url(&self) -> Option<&Url> {
        match self {
            Registry::Cargo(url) => url.as_ref(),
            Registry::Npm(url) => url.as_ref(),
            Registry::Jsr(url) => url.as_ref(),
            Registry::PyPI(url) => url.as_ref(),
        }
    }
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
