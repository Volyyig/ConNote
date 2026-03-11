use serde::{Deserialize, Serialize};

// ── Note ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

// ── Tag ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

// ── Collection ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub rule_json: String,
    pub created_at: String,
    pub updated_at: String,
}

// ── CollectionRule (recursive rule tree for dynamic queries) ──

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CollectionRule {
    And { children: Vec<CollectionRule> },
    Or { children: Vec<CollectionRule> },
    Not { child: Box<CollectionRule> },
    Tag { tag_id: i64 },
}
