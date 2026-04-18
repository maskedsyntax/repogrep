// Parallel file scan: ignore + rayon. Code-like extensions only.

use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

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
pub struct ContextLine {
    pub line_number: u32,
    pub content: String,
    pub is_match: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchResult {
    pub file_path: String,
    pub relative_path: String,
    pub root_hint: String,
    /// Line numbers that contain at least one match (1-based).
    pub lines: Vec<u32>,
    /// Total number of occurrences of the query in this file (respects case_sensitive).
    pub match_count: u32,
    /// Context snippets around the matches.
    pub context: Vec<ContextLine>,
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

fn is_binary(bytes: &[u8]) -> bool {
    let check_len = std::cmp::min(bytes.len(), 1024);
    bytes[..check_len].iter().any(|&b| b == 0)
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

        let mut ignore_builder = GlobSetBuilder::new();
        for pat in ignore_patterns {
            ignore_builder.add(
                Glob::new(pat).map_err(|e| anyhow::anyhow!("Invalid ignore pattern: {}", e))?,
            );
        }
        let ignore_set = ignore_builder.build()?;

        let mut walker_builder = WalkBuilder::new(&root_abs);
        walker_builder.hidden(true); // ignore hidden files/directories (like .git)
        walker_builder.git_ignore(true); // respect .gitignore files
        walker_builder.ignore(true); // respect .ignore files
        
        let walker = walker_builder.build();

        for result in walker {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };

            let p = entry.path();
            if p.is_dir() {
                continue;
            }

            let rel_path_buf = p.strip_prefix(&root_abs).unwrap_or(p);
            let rel_path = normalize_path(rel_path_buf);
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            
            // Check if either the full relative path or the specific filename matches custom ignores
            if ignore_set.is_match(&rel_path) || ignore_set.is_match(name) {
                continue;
            }

            if is_code_file(p, &extension_set) {
                candidates.push(FileCandidate {
                    path: p.to_path_buf(),
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
            let bytes = std::fs::read(&c.path).ok()?;
            if is_binary(&bytes) {
                return None;
            }
            let content = String::from_utf8_lossy(&bytes);
            let mut lines_with_matches = Vec::new();
            let mut match_count: u32 = 0;
            let all_lines: Vec<&str> = content.lines().collect();

            for (i, line) in all_lines.iter().enumerate() {
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
                    lines_with_matches.push(i);
                    match_count += count;
                }
            }
            let processed_now = processed.fetch_add(1, Ordering::Relaxed) + 1;
            if processed_now % 50 == 0 || processed_now == total_candidates {
                on_progress(processed_now, total_candidates);
            }
            if lines_with_matches.is_empty() {
                return None;
            }

            let mut context = Vec::new();
            let mut added_indices = HashSet::new();
            for &idx in &lines_with_matches {
                let start = if idx > 0 { idx - 1 } else { 0 };
                let end = std::cmp::min(idx + 1, all_lines.len() - 1);
                for i in start..=end {
                    if added_indices.insert(i) {
                        context.push(ContextLine {
                            line_number: (i + 1) as u32,
                            content: all_lines[i].to_string(),
                            is_match: lines_with_matches.contains(&i),
                        });
                    }
                }
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
                lines: lines_with_matches.iter().map(|&i| (i + 1) as u32).collect(),
                match_count,
                context,
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
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_search_with_temp_dir() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let root_str = root.to_string_lossy().to_string();

        // Create some files
        std::fs::create_dir_all(root.join(".git")).unwrap();
        let file1_path = root.join("test1.rs");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "fn main() {{\n    println!(\"hello amphi\");\n}}").unwrap();

        let file2_path = root.join("test2.js");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "console.log('hello AMPHI');").unwrap();

        // Create ignored file via gitignore
        let gitignore_path = root.join(".gitignore");
        let mut gitignore = File::create(&gitignore_path).unwrap();
        writeln!(gitignore, "ignored.rs\n").unwrap();

        let ignored_path = root.join("ignored.rs");
        let mut ignored_file = File::create(&ignored_path).unwrap();
        writeln!(ignored_file, "fn main() {{\n    println!(\"hello amphi\");\n}}").unwrap();

        let roots = vec![root_str];
        let exts = DEFAULT_CODE_EXTENSIONS.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        
        // Case insensitive search
        let out = search("amphi", true, false, false, &roots, &[], &exts).unwrap();
        assert_eq!(out.len(), 2, "Should find 2 files containing 'amphi' case-insensitively");
        
        let paths: Vec<_> = out.iter().map(|r| r.relative_path.as_str()).collect();
        assert!(paths.contains(&"test1.rs"));
        assert!(paths.contains(&"test2.js"));
        assert!(!paths.contains(&"ignored.rs"), "Should not find ignored.rs");

        // Verify context
        let test1 = out.iter().find(|r| r.relative_path == "test1.rs").unwrap();
        assert!(!test1.context.is_empty());
        assert!(test1.context.iter().any(|c| c.is_match && c.content.contains("amphi")));

        // Create binary file
        let binary_path = root.join("binary.rs");
        let mut binary_file = File::create(&binary_path).unwrap();
        binary_file.write_all(&[0, 1, 2, 3, 104, 101, 108, 108, 111, 0]).unwrap(); // Contains nulls

        let out3 = search("hello", true, false, false, &roots, &[], &exts).unwrap();
        assert!(!out3.iter().any(|r| r.relative_path == "binary.rs"), "Should skip binary file");
        
        // Custom ignore pattern test
        let ignore_patterns = vec!["test2.js".to_string()];
        let out2 = search("amphi", true, false, false, &roots, &ignore_patterns, &exts).unwrap();
        assert_eq!(out2.len(), 1, "Should ignore test2.js");
        assert_eq!(out2[0].relative_path, "test1.rs");
    }
}
