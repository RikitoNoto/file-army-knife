#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod line_counter;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      line_counter::line_counter::input_start_string,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
