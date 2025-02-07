use std::path::Path;

use crate::package::{package_location::PackageLocation, Package};

pub async fn resolve_python_package_info(path: &Path, location: &PackageLocation) -> Option<Package> {
    todo!()
    // if has_file(path, "requirements.txt").await || has_file(path, "Pipfile").await {
    //     tokio::fs::canonicalize(path)
    //         .await
    //         .map(|p| {
    //             p.components()
    //                 .next_back()
    //                 .map(|c| c.as_os_str().to_string_lossy().to_string())
    //         })
    //         .ok()
    //         .flatten()
    // } else {
    //     None
    // }
}

// async fn has_file(path: &Path, name: &str) -> bool {
//     tokio::fs::metadata(path.join(name))
//         .await
//         .map(|m| m.is_file())
//         .unwrap_or(false)
// }
