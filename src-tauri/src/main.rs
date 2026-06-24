// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    template_tauri_vue_tw_naive_iconfy_ts_lib::run()
}
