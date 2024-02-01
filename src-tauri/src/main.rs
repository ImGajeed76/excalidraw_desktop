// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{thread, time};

fn main() {
  let delay = time::Duration::from_secs(3);
  thread::sleep(delay);

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
