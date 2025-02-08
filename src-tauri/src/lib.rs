#![feature(io_error_more)]
#![feature(exit_status_error)]

use package::package_location::{is_path_valid, resolve_package};
use registry::search_registry;
use reqwest::header::USER_AGENT;
use specta_typescript::Typescript;
use tauri::{
    http::{HeaderMap, HeaderValue},
    Manager,
};
use tauri_specta::{collect_commands, collect_events};

mod package;
mod registry;

struct AppData {
    client: reqwest::Client,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        .error_handling(tauri_specta::ErrorHandlingMode::Result)
        .commands(collect_commands![
            is_path_valid,
            resolve_package,
            search_registry
        ])
        .events(collect_events![]);

    #[cfg(debug_assertions)]
    generate_bindings(&builder);

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            let mut default_headers = HeaderMap::new();
            default_headers.append(USER_AGENT, HeaderValue::from_static("dep_grapher"));
            let client = reqwest::Client::builder()
                .default_headers(default_headers)
                .build()
                .unwrap();

            app.manage(AppData { client });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_bindings(builder: &tauri_specta::Builder) {
    builder
        .export(
            Typescript::default().header("/* eslint-disable */\n// @ts-nocheck\n"),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    // bun prettier --write ../src/bindings.ts
    run_command("bun prettier --write ../src/bindings.ts")
        .expect("failed to run prettier on bindings.ts")
        .status
        .exit_ok()
        .expect("prettier failed running on bindings.ts");

    // cd .. && bun ts-to-zod
    run_command("cd .. && bun ts-to-zod")
        .expect("failed to run ts-to-zod")
        .status
        .exit_ok()
        .expect("ts-to-zod failed");

    // bun prettier --write ../src/bindings.zod.ts
    run_command("bun prettier --write ../src/bindings.zod.ts")
        .expect("failed to run prettier on bindings.zod.ts")
        .status
        .exit_ok()
        .expect("prettier failed running on bindings.zod.ts");
}

fn run_command(command: &str) -> Result<std::process::Output, std::io::Error> {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
    }
}
