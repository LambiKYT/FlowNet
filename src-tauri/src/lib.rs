use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};

use flownet_core::capture::engine::{CaptureEngine, CaptureEvent};

struct AppState {
    engine: Arc<Mutex<CaptureEngine>>,
}

#[tauri::command]
fn list_interfaces() -> Result<Vec<String>, String> {
    CaptureEngine::list_devices()
}

#[tauri::command]
fn start_capture(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    device: String,
) -> Result<(), String> {
    let mut engine = state
        .engine
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    let app_handle = app.clone();

    engine.start(&device, move |event| match event {
        CaptureEvent::PacketBatch(batch) => {
            let _ = app_handle.emit("capture-batch", &batch);
        }
        CaptureEvent::Error(e) => {
            let _ = app_handle.emit("capture-error", &e);
        }
        CaptureEvent::Stopped => {
            let _ = app_handle.emit("capture-stopped", "");
        }
    });

    Ok(())
}

#[tauri::command]
fn stop_capture(state: tauri::State<AppState>) -> Result<(), String> {
    let mut engine = state
        .engine
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    engine.stop();
    Ok(())
}

fn stop_capture_if_running(app: &tauri::AppHandle) {
    if let Some(state) = app.try_state::<AppState>() {
        if let Ok(mut engine) = state.engine.lock() {
            engine.stop();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            engine: Arc::new(Mutex::new(CaptureEngine::new())),
        })
        .invoke_handler(tauri::generate_handler![
            list_interfaces,
            start_capture,
            stop_capture,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                stop_capture_if_running(window.app_handle());
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
