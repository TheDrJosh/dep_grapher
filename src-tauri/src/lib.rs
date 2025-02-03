#![feature(io_error_more)]

use std::{io::ErrorKind, path::PathBuf};

use get_projects::get_projects_in_dir;
use serde::Serialize;

mod get_projects;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_path_valid, get_projects_in_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize)]
enum InvalidPath {
    NotFound,
    NotADirectory,
    InvalidName,
    NotAbsolute,
    Unknown,
}

#[tauri::command]
async fn is_path_valid(path: PathBuf) -> Result<(), InvalidPath> {
    if path.is_relative() {
        return Err(InvalidPath::NotAbsolute);
    }

    let metadata = tokio::fs::metadata(path)
        .await
        .map_err(|err| match err.kind() {
            ErrorKind::NotFound => InvalidPath::NotFound,
            ErrorKind::InvalidFilename => InvalidPath::InvalidName,
            _ => InvalidPath::Unknown,
        })?;

    if !metadata.is_dir() {
        return Err(InvalidPath::NotADirectory);
    }

    Ok(())
}
