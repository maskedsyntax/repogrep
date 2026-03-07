use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;

use crate::models::{Snippet, Version};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS snippets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                code TEXT NOT NULL,
                language TEXT NOT NULL DEFAULT 'plaintext',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL
            );
            CREATE TABLE IF NOT EXISTS snippet_tags (
                snippet_id INTEGER NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
                tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
                PRIMARY KEY (snippet_id, tag_id)
            );
            CREATE TABLE IF NOT EXISTS versions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                snippet_id INTEGER NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
                code TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_snippets_updated ON snippets(updated_at);
            CREATE INDEX IF NOT EXISTS idx_snippet_tags_snippet ON snippet_tags(snippet_id);
            CREATE VIRTUAL TABLE IF NOT EXISTS snippets_fts USING fts5(
                title, code, language, content='snippets', content_rowid='id'
            );
            CREATE TRIGGER IF NOT EXISTS snippets_ai AFTER INSERT ON snippets BEGIN
                INSERT INTO snippets_fts(rowid, title, code, language) VALUES (new.id, new.title, new.code, new.language);
            END;
            CREATE TRIGGER IF NOT EXISTS snippets_ad AFTER DELETE ON snippets BEGIN
                INSERT INTO snippets_fts(snippets_fts, rowid, title, code, language) VALUES('delete', old.id, old.title, old.code, old.language);
            END;
            CREATE TRIGGER IF NOT EXISTS snippets_au AFTER UPDATE ON snippets BEGIN
                INSERT INTO snippets_fts(snippets_fts, rowid, title, code, language) VALUES('delete', old.id, old.title, old.code, old.language);
                INSERT INTO snippets_fts(rowid, title, code, language) VALUES (new.id, new.title, new.code, new.language);
            END;
            "#,
        )?;
        Ok(())
    }

    pub fn get_all_snippets(&self) -> Result<Vec<Snippet>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, code, language, created_at, updated_at FROM snippets ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, u64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (id, title, code, language, created_at, updated_at) = row?;
            let tags = self.get_snippet_tags(id)?;
            let versions = self.get_snippet_versions(id)?;
            out.push(Snippet {
                id,
                title,
                code,
                language,
                tags,
                versions,
                created_at,
                updated_at,
            });
        }
        Ok(out)
    }

    pub fn get_snippets_by_ids(&self, ids: &[u64]) -> Result<Vec<Snippet>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "SELECT id, title, code, language, created_at, updated_at FROM snippets WHERE id IN ({}) ORDER BY updated_at DESC",
            placeholders
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(ids.iter()), |row| {
            Ok((
                row.get::<_, u64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (id, title, code, language, created_at, updated_at) = row?;
            let tags = self.get_snippet_tags(id)?;
            let versions = self.get_snippet_versions(id)?;
            out.push(Snippet {
                id,
                title,
                code,
                language,
                tags,
                versions,
                created_at,
                updated_at,
            });
        }
        Ok(out)
    }

    fn get_snippet_tags(&self, snippet_id: u64) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.name FROM tags t JOIN snippet_tags st ON t.id = st.tag_id WHERE st.snippet_id = ?",
        )?;
        let rows = stmt.query_map([snippet_id], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn get_snippet_versions(&self, snippet_id: u64) -> Result<Vec<Version>> {
        let mut stmt = self.conn.prepare("SELECT timestamp, code FROM versions WHERE snippet_id = ? ORDER BY timestamp DESC LIMIT 20")?;
        let rows = stmt.query_map([snippet_id], |row| {
            Ok(Version {
                timestamp: row.get(0)?,
                code: row.get(1)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn search(&self, query: &str, tags: &[String]) -> Result<Vec<Snippet>> {
        let mut ids = if query.trim().is_empty() {
            let mut stmt = self.conn.prepare("SELECT id FROM snippets ORDER BY updated_at DESC")?;
            let rows = stmt.query_map([], |row| row.get::<_, u64>(0))?;
            rows.collect::<Result<Vec<_>, _>>()?
        } else {
            let pattern = format!("{}*", query.trim());
            let mut stmt = self.conn.prepare(
                "SELECT rowid FROM snippets_fts WHERE snippets_fts MATCH ? ORDER BY rank",
            )?;
            let rows = stmt.query_map([&pattern], |row| row.get::<_, u64>(0))?;
            rows.collect::<Result<Vec<_>, _>>()?
        };

        if !tags.is_empty() {
            let placeholders = tags.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            let sql = format!(
                "SELECT snippet_id FROM snippet_tags st JOIN tags t ON t.id = st.tag_id WHERE t.name IN ({}) GROUP BY snippet_id HAVING COUNT(DISTINCT t.name) = {}",
                placeholders,
                tags.len()
            );
            let mut stmt = self.conn.prepare(&sql)?;
            let matching_ids: std::collections::HashSet<u64> = stmt
                .query_map(rusqlite::params_from_iter(tags.iter()), |row| row.get::<_, u64>(0))?
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .collect();
            ids.retain(|id| matching_ids.contains(id));
        }

        if ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "SELECT id, title, code, language, created_at, updated_at FROM snippets WHERE id IN ({}) ORDER BY updated_at DESC",
            placeholders
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(ids.iter()), |row| {
            Ok((
                row.get::<_, u64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (id, title, code, language, created_at, updated_at) = row?;
            let snippet_tags = self.get_snippet_tags(id)?;
            let versions = self.get_snippet_versions(id)?;
            out.push(Snippet {
                id,
                title,
                code,
                language,
                tags: snippet_tags,
                versions,
                created_at,
                updated_at,
            });
        }
        Ok(out)
    }

    pub fn get_snippet(&self, id: u64) -> Result<Option<Snippet>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, code, language, created_at, updated_at FROM snippets WHERE id = ?",
        )?;
        let mut rows = stmt.query([id])?;
        let row = match rows.next()? {
            Some(r) => r,
            None => return Ok(None),
        };
        let id: u64 = row.get(0)?;
        let title: String = row.get(1)?;
        let code: String = row.get(2)?;
        let language: String = row.get(3)?;
        let created_at: String = row.get(4)?;
        let updated_at: String = row.get(5)?;
        let tags = self.get_snippet_tags(id)?;
        let versions = self.get_snippet_versions(id)?;
        Ok(Some(Snippet {
            id,
            title,
            code,
            language,
            tags,
            versions,
            created_at,
            updated_at,
        }))
    }

    fn ensure_tag(&self, name: &str) -> Result<u64> {
        self.conn.execute("INSERT OR IGNORE INTO tags (name) VALUES (?)", [name])?;
        let id: u64 = self.conn.query_row("SELECT id FROM tags WHERE name = ?", [name], |row| row.get(0))?;
        Ok(id)
    }

    pub fn insert_snippet(
        &self,
        title: &str,
        code: &str,
        language: &str,
        tags: &[String],
    ) -> Result<Snippet> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO snippets (title, code, language, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            params![title, code, language, now, now],
        )?;
        let id = self.conn.last_insert_rowid() as u64;
        for tag in tags {
            let tag_id = self.ensure_tag(tag)?;
            self.conn.execute("INSERT OR IGNORE INTO snippet_tags (snippet_id, tag_id) VALUES (?, ?)", params![id, tag_id])?;
        }
        self.get_snippet(id).map(|o| o.unwrap())
    }

    pub fn update_snippet(
        &self,
        id: u64,
        title: Option<&str>,
        code: Option<&str>,
        language: Option<&str>,
        tags: Option<&[String]>,
    ) -> Result<Snippet> {
        let existing = self.get_snippet(id)?.ok_or_else(|| anyhow::anyhow!("Snippet not found"))?;
        if let Some(_code) = code {
            self.conn.execute(
                "INSERT INTO versions (snippet_id, code, timestamp) VALUES (?, ?, ?)",
                params![id, existing.code, Utc::now().to_rfc3339()],
            )?;
        }
        let title = title.unwrap_or(&existing.title);
        let code = code.unwrap_or(&existing.code);
        let language = language.unwrap_or(&existing.language);
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE snippets SET title = ?, code = ?, language = ?, updated_at = ? WHERE id = ?",
            params![title, code, language, now, id],
        )?;
        self.conn.execute("DELETE FROM snippet_tags WHERE snippet_id = ?", [id])?;
        if let Some(tags) = tags {
            for tag in tags {
                let tag_id = self.ensure_tag(tag)?;
                self.conn.execute("INSERT OR IGNORE INTO snippet_tags (snippet_id, tag_id) VALUES (?, ?)", params![id, tag_id])?;
            }
        }
        self.get_snippet(id).map(|o| o.unwrap())
    }

    pub fn delete_snippet(&self, id: u64) -> Result<()> {
        self.conn.execute("DELETE FROM snippet_tags WHERE snippet_id = ?", [id])?;
        self.conn.execute("DELETE FROM versions WHERE snippet_id = ?", [id])?;
        self.conn.execute("DELETE FROM snippets WHERE id = ?", [id])?;
        Ok(())
    }

    pub fn index_directory(&self, dir: &str) -> Result<()> {
        use std::fs;
        use walkdir::WalkDir;

        let exts = [
            "rs", "js", "ts", "vue", "py", "java", "kt", "go", "sql", "sh", "bash", "json", "toml", "yaml", "yml", "md", "html", "css", "scss",
        ];
        for entry in WalkDir::new(dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !exts.contains(&ext) {
                continue;
            }
            let title = path.file_name().and_then(|n| n.to_str()).unwrap_or("file").to_string();
            let code = fs::read_to_string(path).unwrap_or_default();
            let language = match ext {
                "rs" => "rust",
                "js" => "javascript",
                "ts" => "typescript",
                "vue" => "vue",
                "py" => "python",
                "java" => "java",
                "kt" => "kotlin",
                "go" => "go",
                "sql" => "sql",
                "sh" | "bash" => "bash",
                "json" => "json",
                "toml" => "toml",
                "yaml" | "yml" => "yaml",
                "md" => "markdown",
                "html" => "html",
                "css" => "css",
                "scss" => "scss",
                _ => "plaintext",
            };
            let _ = self.insert_snippet(&title, &code, language, &[]);
        }
        Ok(())
    }
}
