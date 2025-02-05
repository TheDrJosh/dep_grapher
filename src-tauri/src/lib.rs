#![feature(io_error_more)]

use std::{io::ErrorKind, path::PathBuf};

use get_projects::get_projects_in_dir;
use registrys::search_registry;
use serde::Serialize;
use specta::Type;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, collect_events};

mod get_projects;
mod registrys;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        .error_handling(tauri_specta::ErrorHandlingMode::Result)
        .commands(collect_commands![is_path_valid, get_projects_in_dir, search_registry])
        .events(collect_events![]);

    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Type)]
enum InvalidPath {
    NotFound,
    NotADirectory,
    InvalidName,
    NotAbsolute,
    Unknown,
}

#[tauri::command]
#[specta::specta]
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
