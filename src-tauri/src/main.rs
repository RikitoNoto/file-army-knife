#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod util;
mod line_counter;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::commands::commands::get_enumrate_files,
    ])
    .run(
      tauri::generate_context!()
    )
    .expect("error while running tauri application");
}
