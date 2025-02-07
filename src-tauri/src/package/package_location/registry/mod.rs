use serde::{Deserialize, Serialize};
use specta::Type;

use crate::registry::{resolve::RegistryResolveError, Registry};

use super::Package;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct RegistryPackageLocation {
    pub registry: Registry,
    pub name: String,
    pub version: String,
}


impl RegistryPackageLocation {
    pub async fn resolve(&self) -> Result<Package, RegistryResolveError> {
        self.registry.resolve(&self.name, &self.version).await
    }
}