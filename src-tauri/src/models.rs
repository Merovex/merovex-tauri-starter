//! Data shapes shared with the frontend. These serialize to JSON and are the
//! return/argument types of Tauri commands.
//!
//! `Item` is a PLACEHOLDER — replace it with your real domain models.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub body: String,
    pub position: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentFile {
    pub id: String,
    pub path: String,
    pub title: String,
    pub last_opened: String,
}
