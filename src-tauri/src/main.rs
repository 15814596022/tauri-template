#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
// Create the command:
// This command must be async so that it doesn't run on the main thread.
#[tauri::command]
async fn show_main(window: tauri::Window) {
  window.get_window("main").unwrap().show().unwrap();
  #[cfg(debug_assertions)]
  window.get_window("main").unwrap().open_devtools();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![show_main])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
