mod db;
mod models;

use anyhow::Result;
use models::Snippet;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

struct AppState {
    db: Mutex<Option<db::Database>>,
}

#[tauri::command]
fn get_snippets(state: State<AppState>) -> Result<Vec<Snippet>, String> {
    let guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or("Database not initialized")?;
    db.get_all_snippets().map_err(|e| e.to_string())
}

#[tauri::command]
fn search_snippets(
    state: State<AppState>,
    query: String,
    tags: Option<Vec<String>>,
    _fuzzy: Option<bool>,
) -> Result<Vec<Snippet>, String> {
    let guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or("Database not initialized")?;
    db.search(&query, tags.as_deref().unwrap_or(&[])).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_snippet(state: State<AppState>, id: u64) -> Result<Option<Snippet>, String> {
    let guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or("Database not initialized")?;
    db.get_snippet(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn add_snippet(
    state: State<AppState>,
    title: String,
    code: String,
    language: String,
    tags: Option<Vec<String>>,
) -> Result<Snippet, String> {
    let guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or("Database not initialized")?;
    db.insert_snippet(&title, &code, &language, tags.as_deref().unwrap_or(&[]))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_snippet(
    state: State<AppState>,
    id: u64,
    title: Option<String>,
    code: Option<String>,
    language: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Snippet, String> {
    let guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or("Database not initialized")?;
    db.update_snippet(id, title.as_deref(), code.as_deref(), language.as_deref(), tags.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_snippet(state: State<AppState>, id: u64) -> Result<(), String> {
    let guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or("Database not initialized")?;
    db.delete_snippet(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn index_directories(
    app: AppHandle,
    state: State<AppState>,
    dirs: Vec<String>,
) -> Result<(), String> {
    let total = dirs.len();
    for (i, dir) in dirs.iter().enumerate() {
        let guard = state.db.lock().map_err(|e| e.to_string())?;
        if let Some(ref db) = *guard {
            if let Err(e) = db.index_directory(dir) {
                let _ = app.emit("index_error", e.to_string());
            }
        }
        drop(guard);
        let progress = ((i + 1) as f64 / total as f64) * 100.0;
        let _ = app.emit(
            "index_progress",
            serde_json::json!({ "progress": progress, "current": dir }),
        );
    }
    let _ = app.emit(
        "index_progress",
        serde_json::json!({ "progress": 100.0, "done": true }),
    );
    Ok(())
}

#[tauri::command]
fn import_from_clipboard(state: State<AppState>, content: String, language: Option<String>) -> Result<Snippet, String> {
    let guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or("Database not initialized")?;
    let lang = language.unwrap_or_else(|| "plaintext".to_string());
    let title = content.lines().next().unwrap_or("Clipboard").to_string();
    db.insert_snippet(&title, &content, &lang, &[]).map_err(|e| e.to_string())
}

#[tauri::command]
fn export_snippets(state: State<AppState>, ids: Option<Vec<u64>>) -> Result<String, String> {
    let guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or("Database not initialized")?;
    let snippets = if let Some(ids) = ids {
        db.get_snippets_by_ids(&ids).map_err(|e| e.to_string())?
    } else {
        db.get_all_snippets().map_err(|e| e.to_string())?
    };
    serde_json::to_string_pretty(&snippets).map_err(|e| e.to_string())
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let path = app_data_dir(&app.handle())?;
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            let db_path = path.join("snippets.db");
            let db = db::Database::open(&db_path).map_err(|e| e.to_string())?;
            app.manage(AppState {
                db: Mutex::new(Some(db)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_snippets,
            search_snippets,
            get_snippet,
            add_snippet,
            update_snippet,
            delete_snippet,
            index_directories,
            import_from_clipboard,
            export_snippets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
