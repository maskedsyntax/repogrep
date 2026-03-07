use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: u64,
    pub title: String,
    pub code: String,
    pub language: String,
    pub tags: Vec<String>,
    pub versions: Vec<Version>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub timestamp: String,
    pub code: String,
}
