// repogrep – local code search across project directories
// Backend: project paths (JSON), search via walkdir + str::contains, read_file_content

mod search;
use search::MatchResult;

use std::path::PathBuf;
use std::process::Command;
use tauri::{AppHandle, Emitter, Manager};

const PATHS_FILENAME: &str = "repogrep_paths.json";
const IGNORES_FILENAME: &str = "repogrep_ignores.json";
const EXTENSIONS_FILENAME: &str = "repogrep_extensions.json";

/// Strip file:// or file:/// prefix so Path::new(...).is_dir() works.
fn normalize_path_string(s: &str) -> String {
    let s = s.trim();
    if s.starts_with("file:///") {
        s[7..].to_string()
    } else if s.starts_with("file://") {
        s[6..].to_string()
    } else {
        s.to_string()
    }
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path().app_data_dir().map_err(|e| e.to_string())
}

fn paths_file(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join(PATHS_FILENAME))
}

fn ignores_file(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join(IGNORES_FILENAME))
}

fn extensions_file(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join(EXTENSIONS_FILENAME))
}

#[tauri::command]
fn get_ignore_patterns(app: AppHandle) -> Result<Vec<String>, String> {
    let pf = ignores_file(&app)?;
    if pf.exists() {
        let s = std::fs::read_to_string(&pf).map_err(|e| e.to_string())?;
        Ok(serde_json::from_str(&s).unwrap_or_default())
    } else {
        // Default ignores if no file exists
        Ok(vec![
            "node_modules".to_string(),
            "target".to_string(),
            "build".to_string(),
            ".git".to_string(),
            "__pycache__".to_string(),
        ])
    }
}

#[tauri::command]
fn add_ignore_pattern(app: AppHandle, pattern: String) -> Result<(), String> {
    let mut list = get_ignore_patterns(app.clone())?;
    if !list.contains(&pattern) {
        list.push(pattern);
        let pf = ignores_file(&app)?;
        std::fs::write(&pf, serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn remove_ignore_pattern(app: AppHandle, pattern: String) -> Result<(), String> {
    let list = get_ignore_patterns(app.clone())?;
    let new_list: Vec<String> = list.into_iter().filter(|p| p != &pattern).collect();
    let pf = ignores_file(&app)?;
    std::fs::write(&pf, serde_json::to_string_pretty(&new_list).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_code_extensions(app: AppHandle) -> Result<Vec<String>, String> {
    let pf = extensions_file(&app)?;
    if pf.exists() {
        let s = std::fs::read_to_string(&pf).map_err(|e| e.to_string())?;
        Ok(serde_json::from_str(&s).unwrap_or_default())
    } else {
        Ok(search::DEFAULT_CODE_EXTENSIONS
            .iter()
            .map(|e| e.to_string())
            .collect())
    }
}

#[tauri::command]
fn add_code_extension(app: AppHandle, extension: String) -> Result<(), String> {
    let normalized = extension.trim().trim_start_matches('.').to_lowercase();
    if normalized.is_empty() {
        return Ok(());
    }
    let mut list = get_code_extensions(app.clone())?;
    if !list.contains(&normalized) {
        list.push(normalized);
        list.sort();
        let pf = extensions_file(&app)?;
        std::fs::write(&pf, serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn remove_code_extension(app: AppHandle, extension: String) -> Result<(), String> {
    let normalized = extension.trim().trim_start_matches('.').to_lowercase();
    let list = get_code_extensions(app.clone())?;
    let new_list: Vec<String> = list.into_iter().filter(|e| e != &normalized).collect();
    let pf = extensions_file(&app)?;
    std::fs::write(&pf, serde_json::to_string_pretty(&new_list).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_project_paths(app: AppHandle) -> Result<Vec<ProjectPath>, String> {
    let path = paths_file(&app)?;
    let raw: Vec<String> = if path.exists() {
        let s = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        vec![]
    };
    Ok(raw
        .into_iter()
        .map(|p| {
            let path = normalize_path_string(&p);
            ProjectPath {
                path: path.clone(),
                root_hint: PathBuf::from(&path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
            }
        })
        .collect())
}

#[derive(serde::Serialize)]
struct ProjectPath {
    path: String,
    root_hint: String,
}

#[tauri::command]
fn add_project_path(app: AppHandle, path: String) -> Result<(), String> {
    let path = normalize_path_string(&path);
    if path.is_empty() {
        return Ok(());
    }
    let p = paths_file(&app)?;
    let mut list: Vec<String> = if p.exists() {
        let s = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        vec![]
    };
    if !list.contains(&path) {
        list.push(path);
        std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
        std::fs::write(&p, serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn remove_project_path(app: AppHandle, path: String) -> Result<(), String> {
    let pf = paths_file(&app)?;
    let list: Vec<String> = if pf.exists() {
        let s = std::fs::read_to_string(&pf).map_err(|e| e.to_string())?;
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        return Ok(());
    };
    let new_list: Vec<String> = list.into_iter().filter(|p| p != &path).collect();
    std::fs::write(&pf, serde_json::to_string_pretty(&new_list).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchSnippetArgs {
    query: String,
    exact: bool,
    #[serde(alias = "caseSensitive")]
    case_sensitive: bool,
    #[serde(alias = "isRegex")]
    is_regex: bool,
    paths_override: Option<Vec<String>>,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchProgressPayload {
    processed: usize,
    total: usize,
}

#[tauri::command]
async fn search_snippet(args: SearchSnippetArgs, app: AppHandle) -> Result<Vec<MatchResult>, String> {
    let paths: Vec<String> = if let Some(override_paths) = args.paths_override {
        override_paths
    } else {
        let pf = paths_file(&app)?;
        if !pf.exists() {
            return Ok(vec![]);
        }
        let s = std::fs::read_to_string(&pf).map_err(|e| e.to_string())?;
        serde_json::from_str(&s).unwrap_or_default()
    };
    if paths.is_empty() || args.query.trim().is_empty() {
        return Ok(vec![]);
    }
    let paths: Vec<String> = paths.into_iter().map(|p| normalize_path_string(&p)).collect();
    let q = args.query.trim().to_string();
    let case_sensitive = args.case_sensitive;
    let is_regex = args.is_regex;
    let exact = args.exact;
    let ignores = get_ignore_patterns(app.clone())?;
    let code_extensions = get_code_extensions(app.clone())?;
    let app_for_progress = app.clone();
    tokio::task::spawn_blocking(move || {
        search::search_with_progress(&q, exact, case_sensitive, is_regex, &paths, &ignores, &code_extensions, |processed, total| {
            let _ = app_for_progress.emit(
                "search-progress",
                SearchProgressPayload { processed, total },
            );
        })
    })
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn read_file_content(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_file_in_editor(path: String) -> Result<(), String> {
    let p = normalize_path_string(&path);
    if p.trim().is_empty() {
        return Err("Empty file path".to_string());
    }
    if !std::path::Path::new(&p).exists() {
        return Err(format!("File not found: {}", p));
    }

    #[cfg(target_os = "macos")]
    let status = Command::new("open").arg(&p).status().map_err(|e| e.to_string())?;

    #[cfg(target_os = "linux")]
    let status = Command::new("xdg-open").arg(&p).status().map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    let status = Command::new("cmd")
        .args(["/C", "start", "", &p])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Failed to open file: {}", p))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let path = app_data_dir(&app.handle())?;
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_project_paths,
            add_project_path,
            remove_project_path,
            get_ignore_patterns,
            add_ignore_pattern,
            remove_ignore_pattern,
            get_code_extensions,
            add_code_extension,
            remove_code_extension,
            search_snippet,
            read_file_content,
            open_file_in_editor,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
