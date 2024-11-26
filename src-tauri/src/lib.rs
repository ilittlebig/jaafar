#[cfg(target_os = "macos")]
#[macro_use]
extern crate cocoa;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod plugins;

#[cfg(target_os = "macos")]
use plugins::tauri_traffic_light_positioner_plugin;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init());

    #[cfg(target_os = "macos")]
    let builder = builder.plugin(tauri_traffic_light_positioner_plugin::init());

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
