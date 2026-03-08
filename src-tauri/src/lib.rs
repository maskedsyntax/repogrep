// repogrep – local code search across project directories
// Backend: project paths (JSON), search via walkdir + str::contains, read_file_content

mod search;
use search::MatchResult;

use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const PATHS_FILENAME: &str = "repogrep_paths.json";

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
    paths_override: Option<Vec<String>>,
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
    let exact = args.exact;
    tokio::task::spawn_blocking(move || search::search(&q, exact, case_sensitive, &paths))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn read_file_content(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
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
            search_snippet,
            read_file_content,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
