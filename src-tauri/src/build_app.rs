use tauri::Builder as TauriBuilder;
use tauri::{App, generate_context};

pub fn build(builder: TauriBuilder<tauri::Wry>) -> tauri::Result<App> {
    builder.build(generate_context!())
}
