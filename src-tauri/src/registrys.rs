use log::error;
use log::info;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use crate::AppData;

#[derive(Debug, Deserialize, Type)]
pub enum Registry {
    Cargo,
    Npm,
    Jsr,
    PyPI,
}

impl Registry {
    fn default_url(&self) -> Url {
        match self {
            Registry::Cargo => "https://crates.io",
            Registry::Npm => "https://registry.npmjs.org/",
            Registry::Jsr => "https://api.jsr.io",
            Registry::PyPI => "https://pypi.org/simple/",
        }
        .parse()
        .unwrap()
    }
}

#[derive(Debug, Serialize, Type)]
pub enum SearchRegistryError {
    UrlParseError,
    NetworkError,
    ServerError,
    ParseError,
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

const NUMBER_OF_RESULTS: usize = 5;

#[tauri::command]
#[specta::specta]
pub async fn search_registry(
    search: String,
    reg: Registry,
    custom_url: Option<String>,
    state: State<'_, AppData>,
) -> Result<Vec<String>, SearchRegistryError> {
    info!("Searching {:?} for {}", reg, search);

    let base_url = custom_url
        .map(|url| url.parse())
        .unwrap_or(Ok(reg.default_url()))
        .map_err(|e| {
            error!("{}", e);
            SearchRegistryError::UrlParseError
        })?;

    let search_url = match reg {
        Registry::Cargo => {
            let mut search_url = base_url.join("api/v1/crates").unwrap();

            search_url
                .query_pairs_mut()
                .append_pair("q", &search)
                .append_pair("per_page", &NUMBER_OF_RESULTS.to_string());

            search_url
        }
        Registry::Npm => {
            if search.len() < 2 {
                return Ok(vec![]);
            }

            let mut search_url = base_url.join("-/v1/search").unwrap();

            search_url
                .query_pairs_mut()
                .append_pair("text", &search)
                .append_pair("size", &NUMBER_OF_RESULTS.to_string());

            search_url
        }
        Registry::Jsr => {
            let mut search_url = base_url.join("packages").unwrap();

            search_url
                .query_pairs_mut()
                .append_pair("query", &search)
                .append_pair("limit", &NUMBER_OF_RESULTS.to_string());

            search_url
        }
        Registry::PyPI => todo!(),
    };

    let res = state
        .client
        .get(search_url)
        .send()
        .await
        .map_err(|e| {
            error!("{}", e);
            SearchRegistryError::NetworkError
        })?
        .error_for_status()
        .map_err(|e| {
            error!("{}", e);
            SearchRegistryError::ServerError
        })?;

    let search_results = match reg {
        Registry::Cargo => res
            .json::<CargoSearchResult>()
            .await
            .map_err(|e| {
                error!("{}", e);
                SearchRegistryError::ParseError
            })?
            .crates
            .into_iter()
            .map(|crate_| crate_.name)
            .collect::<Vec<_>>(),
        Registry::Npm => res
            .json::<NpmSearchResult>()
            .await
            .map_err(|e| {
                error!("{}", e);
                SearchRegistryError::ParseError
            })?
            .objects
            .into_iter()
            .map(|object| object.package.name)
            .collect::<Vec<_>>(),
        Registry::Jsr => res
            .json::<JsrSearchResult>()
            .await
            .map_err(|e| {
                error!("{}", e);
                SearchRegistryError::ParseError
            })?
            .items
            .into_iter()
            .map(|package| package.name)
            .collect::<Vec<_>>(),
        Registry::PyPI => todo!(),
    };

    Ok(search_results)
}
