use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::Url;

use super::{Registry, RegistryType};

#[derive(Debug, Serialize, Type, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum SearchRegistryError {
    #[error("network error: {0}")]
    Network(#[serde(skip)] reqwest::Error),
    #[error("server error: {0}")]
    Server(#[serde(skip)] reqwest::Error),
    #[error("parse error: {0}")]
    Parse(#[serde(skip)] reqwest::Error),
}

#[derive(Debug, Serialize, Type)]
pub struct SearchResult {
    name: String,
}

const NUMBER_OF_RESULTS: usize = 4;

impl Registry {
    fn search_url(&self, query: &str) -> Url {
        let base_url = self
            .custom_url
            .clone()
            .unwrap_or(self.registry_type.default_url());

        match self.registry_type {
            RegistryType::Cargo => {
                let mut search_url = base_url.join("api/v1/crates").unwrap();

                search_url
                    .query_pairs_mut()
                    .append_pair("q", query)
                    .append_pair("per_page", &(NUMBER_OF_RESULTS + 1).to_string());

                search_url
            }
            RegistryType::Npm => {
                let mut search_url = base_url.join("-/v1/search").unwrap();

                search_url
                    .query_pairs_mut()
                    .append_pair("text", query)
                    .append_pair("size", &(NUMBER_OF_RESULTS + 1).to_string());

                search_url
            }
            RegistryType::Jsr => {
                let mut search_url = base_url.join("packages").unwrap();

                search_url
                    .query_pairs_mut()
                    .append_pair("query", query)
                    .append_pair("limit", &(NUMBER_OF_RESULTS + 1).to_string());

                search_url
            }
            RegistryType::PyPI => todo!(),
        }
    }

    pub async fn search(
        &self,
        query: &str,
        client: Option<Client>,
    ) -> Result<Vec<SearchResult>, SearchRegistryError> {
        info!("Searching {:?} for {}", self, query);

        if matches!(self.registry_type, RegistryType::Npm) && query.len() < 2 {
            return Ok(vec![]);
        }

        let search_url = self.search_url(query);

        let res = client
            .unwrap_or_default()
            .get(search_url)
            .send()
            .await
            .map_err(|e| {
                error!("{}", e);
                SearchRegistryError::Network(e)
            })?
            .error_for_status()
            .map_err(|e| {
                error!("{}", e);
                SearchRegistryError::Server(e)
            })?;

        let search_results = match self.registry_type {
            RegistryType::Cargo => res
                .json::<CargoSearchResult>()
                .await
                .map_err(|e| {
                    error!("{}", e);
                    SearchRegistryError::Parse(e)
                })?
                .crates
                .into_iter()
                .map(|crate_| SearchResult { name: crate_.name })
                .collect::<Vec<_>>(),
            RegistryType::Npm => res
                .json::<NpmSearchResult>()
                .await
                .map_err(|e| {
                    error!("{}", e);
                    SearchRegistryError::Parse(e)
                })?
                .objects
                .into_iter()
                .map(|object| SearchResult {
                    name: object.package.name,
                })
                .collect::<Vec<_>>(),
            RegistryType::Jsr => res
                .json::<JsrSearchResult>()
                .await
                .map_err(|e| {
                    error!("{}", e);
                    SearchRegistryError::Parse(e)
                })?
                .items
                .into_iter()
                .map(|package| SearchResult { name: package.name })
                .collect::<Vec<_>>(),
            RegistryType::PyPI => todo!(),
        };

        Ok(search_results
            .into_iter()
            .filter(|res| res.name != query)
            .take(NUMBER_OF_RESULTS)
            .collect())
    }
}

#[derive(Debug, Deserialize)]
struct CargoSearchResult {
    crates: Vec<CargoCrate>,
}

#[derive(Debug, Deserialize)]
struct CargoCrate {
    name: String,
}

#[derive(Debug, Deserialize)]
struct NpmSearchResult {
    objects: Vec<NpmObject>,
}

#[derive(Debug, Deserialize)]
struct NpmObject {
    package: NpmPackage,
}

#[derive(Debug, Deserialize)]
struct NpmPackage {
    name: String,
}

#[derive(Debug, Deserialize)]
struct JsrSearchResult {
    items: Vec<JsrPackage>,
}

#[derive(Debug, Deserialize)]
struct JsrPackage {
    name: String,
}
