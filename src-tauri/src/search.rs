// Parallel file scan: walkdir + rayon. Code-like extensions only.

use globset::{Glob, GlobSetBuilder};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

/// Extensions we consider "code" for search. Lowercase.
pub const DEFAULT_CODE_EXTENSIONS: &[&str] = &[
    "rs", "vue", "js", "ts", "jsx", "tsx", "mjs", "cjs",
    "dart", "py", "go", "rb", "java", "kt", "kts", "c", "h", "cpp", "hpp", "cc", "cxx",
    "cs", "php", "swift", "scala", "r", "sql", "sh", "bash", "zsh",
    "html", "htm", "css", "scss", "sass", "less",
    "json", "yaml", "yml", "toml", "xml", "md", "markdown",
    "lua", "vim", "el", "ex", "exs", "erl", "hs", "fs", "fsx", "ml", "mli",
    "txt", "log", "conf", "ini", "env", "text", "sh", "bash", "zsh", "pl", "pyw",
];

#[derive(Debug, Clone, Serialize)]
pub struct MatchResult {
    pub file_path: String,
    pub relative_path: String,
    pub root_hint: String,
    /// Line numbers that contain at least one match (1-based).
    pub lines: Vec<u32>,
    /// Total number of occurrences of the query in this file (respects case_sensitive).
    pub match_count: u32,
}

fn is_code_file(path: &Path, code_extensions: &HashSet<String>) -> bool {
    // Check extension
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if code_extensions.contains(&ext.to_lowercase()) {
            return true;
        }
    }
    
    // Check for common files without extensions
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let n = name.to_lowercase();
        if n == "license" || n == "dockerfile" || n == "makefile" || n.starts_with(".env") {
            return true;
        }
    }

    false
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn load_gitignore_patterns(root_abs: &Path) -> Vec<String> {
    let gitignore_path = root_abs.join(".gitignore");
    let content = match std::fs::read_to_string(gitignore_path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();
    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Negation patterns are not supported in this first pass.
        if line.starts_with('!') {
            continue;
        }

        let mut pat = line.trim_start_matches('/').to_string();
        if pat.ends_with('/') {
            pat = format!("{}**", pat);
        }
        // .gitignore-style bare names should match nested paths too.
        if !pat.contains('/') {
            pat = format!("**/{}", pat);
        }
        out.push(pat);
    }
    out
}

struct FileCandidate {
    path: PathBuf,
    root_hint: String,
    root_canonical: PathBuf,
}

use regex::RegexBuilder;

#[allow(dead_code)]
pub fn search(
    query: &str,
    _exact: bool,
    case_sensitive: bool,
    is_regex: bool,
    root_paths: &[String],
    ignore_patterns: &[String],
    code_extensions: &[String],
) -> anyhow::Result<Vec<MatchResult>> {
    search_with_progress(
        query,
        _exact,
        case_sensitive,
        is_regex,
        root_paths,
        ignore_patterns,
        code_extensions,
        |_processed, _total| {},
    )
}

pub fn search_with_progress<F>(
    query: &str,
    _exact: bool,
    case_sensitive: bool,
    is_regex: bool,
    root_paths: &[String],
    ignore_patterns: &[String],
    code_extensions: &[String],
    on_progress: F,
) -> anyhow::Result<Vec<MatchResult>>
where
    F: Fn(usize, usize) + Send + Sync,
{
    let extension_set: HashSet<String> = code_extensions
        .iter()
        .map(|e| e.trim().trim_start_matches('.').to_lowercase())
        .filter(|e| !e.is_empty())
        .collect();

    let re = if is_regex {
        Some(
            RegexBuilder::new(query)
                .case_insensitive(!case_sensitive)
                .build()
                .map_err(|e| anyhow::anyhow!("Invalid regex: {}", e))?,
        )
    } else {
        None
    };

    let query_lower = if !is_regex && !case_sensitive {
        Some(query.to_lowercase())
    } else {
        None
    };
    let query = query.to_string();

    let mut candidates: Vec<FileCandidate> = Vec::new();
    for root in root_paths {
        let root_path = Path::new(root);
        // Resolve to absolute so relative paths and file://-stripped paths work regardless of cwd
        let root_abs = match root_path.canonicalize() {
            Ok(p) => p,
            Err(_) => continue, // path doesn't exist or not a directory
        };
        let root_hint = root_abs
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let gitignore_patterns = load_gitignore_patterns(&root_abs);
        let mut ignore_builder = GlobSetBuilder::new();
        for pat in ignore_patterns {
            ignore_builder.add(
                Glob::new(pat).map_err(|e| anyhow::anyhow!("Invalid ignore pattern: {}", e))?,
            );
        }
        for pat in &gitignore_patterns {
            ignore_builder.add(
                Glob::new(pat).map_err(|e| anyhow::anyhow!("Invalid .gitignore pattern: {}", e))?,
            );
        }
        let ignore_set = ignore_builder.build()?;

        let walker = WalkDir::new(&root_abs)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                // Never skip the root directory itself
                if e.depth() == 0 {
                    return true;
                }
                let p = e.path();
                let rel_path_buf = p.strip_prefix(&root_abs).unwrap_or(p);
                let rel_path = normalize_path(rel_path_buf);
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                
                // Check if either the full relative path or the specific filename matches
                if ignore_set.is_match(&rel_path) || ignore_set.is_match(name) {
                    return false;
                }

                if p.is_dir() {
                    !name.starts_with('.')
                } else {
                    is_code_file(p, &extension_set)
                }
            });

        for entry in walker.filter_map(|e| e.ok()) {
            let path = entry.path().to_path_buf();
            if path.is_file() && is_code_file(&path, &extension_set) {
                candidates.push(FileCandidate {
                    path,
                    root_hint: root_hint.clone(),
                    root_canonical: root_abs.clone(),
                });
            }
        }
    }

    let total_candidates = candidates.len();
    on_progress(0, total_candidates);
    let processed = AtomicUsize::new(0);

    let all_matches: Vec<MatchResult> = candidates
        .par_iter()
        .filter_map(|c| {
            let content = std::fs::read_to_string(&c.path).ok()?;
            let mut lines = Vec::new();
            let mut match_count: u32 = 0;
            for (i, line) in content.lines().enumerate() {
                let line_num = (i + 1) as u32;
                let (found, count) = if let Some(ref r) = re {
                    let n = r.find_iter(line).count() as u32;
                    (n > 0, n)
                } else if case_sensitive {
                    let n = line.matches(&query).count() as u32;
                    (n > 0, n)
                } else {
                    let lower = query_lower.as_ref().unwrap();
                    let n = line.to_lowercase().matches(lower.as_str()).count() as u32;
                    (n > 0, n)
                };
                if found {
                    lines.push(line_num);
                    match_count += count;
                }
            }
            let processed_now = processed.fetch_add(1, Ordering::Relaxed) + 1;
            if processed_now % 50 == 0 || processed_now == total_candidates {
                on_progress(processed_now, total_candidates);
            }
            if lines.is_empty() {
                return None;
            }
            let file_path = normalize_path(&c.path);
            let relative = c
                .path
                .strip_prefix(&c.root_canonical)
                .map(|p| normalize_path(p))
                .unwrap_or_else(|_| file_path.clone());
            Some(MatchResult {
                file_path,
                relative_path: relative,
                root_hint: c.root_hint.clone(),
                lines,
                match_count,
            })
        })
        .collect();

    let processed_end = processed.load(Ordering::Relaxed);
    if processed_end < total_candidates {
        on_progress(total_candidates, total_candidates);
    }

    let mut sorted = all_matches;
    sorted.sort_by(|a, b| {
        a.root_hint
            .cmp(&b.root_hint)
            .then_with(|| a.relative_path.cmp(&b.relative_path))
    });
    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run with: cargo test test_search_amphi_repo -- --ignored
    /// Verifies core search logic finds "amphi" and "import" in ~/maskedsyntax/amphi
    #[test]
    #[ignore]
    fn test_search_amphi_repo() {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        let root = format!("{}/maskedsyntax/amphi", home);
        if !std::path::Path::new(&root).exists() {
            eprintln!("Skip: {} not found", root);
            return;
        }
        let roots = vec![root];

        // Search "amphi" (e.g. in README.md)
        let exts = DEFAULT_CODE_EXTENSIONS.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let out = search("amphi", true, false, false, &roots, &[], &exts).unwrap();
        assert!(!out.is_empty(), "search 'amphi' should find files (e.g. README.md) in {}", roots[0]);
        let has_readme = out.iter().any(|r| r.relative_path.contains("README") || r.file_path.contains("README"));
        assert!(has_readme, "expected at least README.md in results; got {:?}", out.iter().map(|r| r.relative_path.as_str()).collect::<Vec<_>>());

        // Search "import" (common in code)
        let out2 = search("import", true, false, false, &roots, &[], &exts).unwrap();
        assert!(!out2.is_empty(), "search 'import' should find files in {}", roots[0]);
    }
}
