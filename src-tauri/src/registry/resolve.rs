use serde::Serialize;
use specta::Type;

use crate::package::Package;

use super::Registry;

#[derive(Debug, Clone, Serialize, Type, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum RegistryResolveError {}

impl Registry {
    pub async fn resolve(
        &self,
        name: &str,
        version: &str,
    ) -> Result<Package, RegistryResolveError> {
        todo!()
    }
}
