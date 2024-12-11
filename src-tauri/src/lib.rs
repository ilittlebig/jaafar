#[cfg(target_os = "macos")]
#[macro_use]
extern crate cocoa;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod plugins;
mod signups;
mod services;
mod captcha_solvers;
mod phone_numbers;
mod utils;
mod data;

use plugins::tauri_traffic_light_positioner_plugin;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_traffic_light_positioner_plugin::init())

        .invoke_handler(tauri::generate_handler![
            signups::sabrina_hallenstadion,
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
