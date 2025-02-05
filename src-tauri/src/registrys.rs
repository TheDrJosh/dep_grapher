use log::error;
use log::info;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use reqwest::Method;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use specta::Type;

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
            Registry::Npm => todo!(),
            Registry::Jsr => todo!(),
            Registry::PyPI => todo!(),
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

#[tauri::command]
#[specta::specta]
pub async fn search_registry(
    search: String,
    reg: Registry,
    custom_url: Option<String>,
) -> Result<Vec<String>, SearchRegistryError> {
    info!("Searching {:?} for {}", reg, search);

    match reg {
        Registry::Cargo => {
            let base_url = custom_url
                .map(|url| url.parse())
                .unwrap_or(Ok(reg.default_url()))
                .map_err(|e| {
                    error!("{}", e);
                    SearchRegistryError::UrlParseError
                })?;

            let mut search_url = base_url.join("api/v1/crates").unwrap();

            search_url
                .query_pairs_mut()
                .append_pair("q", &search)
                .append_pair("per_page", "25");

            let res = Client::new()
                .get(search_url)
                .header(USER_AGENT, "dep_grapher")
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
                })?
                .json::<CargoSearchResult>()
                .await
                .map_err(|e| {
                    error!("{}", e);
                    SearchRegistryError::ParseError
                })?;

            Ok(res
                .crates
                .into_iter()
                .map(|crate_| crate_.name)
                .collect::<Vec<_>>())
        }
        Registry::Npm => todo!(),
        Registry::Jsr => todo!(),
        Registry::PyPI => todo!(),
    }
}
