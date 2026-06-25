// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn calc(number1: usize, number2: usize) -> Result<String, String> {
    if number2 == 0 {
        return Err("cannot be divided by 0".to_string());
    }
    Ok(format!("{}", number1 / number2))
}

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static
{
    tauri::generate_handler![calc, greet]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _handler = generate_handlers();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
