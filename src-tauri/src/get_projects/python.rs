use std::path::Path;

pub async fn read_python_project_info(path: &Path) -> Option<String> {
    if has_file(path, "requirements.txt").await || has_file(path, "Pipfile").await {
        tokio::fs::canonicalize(path)
            .await
            .map(|p| {
                p.components()
                    .last()
                    .map(|c| c.as_os_str().to_string_lossy().to_string())
            })
            .ok()
            .flatten()
    } else {
        None
    }
}

async fn has_file(path: &Path, name: &str) -> bool {
    tokio::fs::metadata(path.join(name))
        .await
        .map(|m| m.is_file())
        .unwrap_or(false)
}
