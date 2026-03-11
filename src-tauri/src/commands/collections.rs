use crate::db::DbState;
use crate::models::{Collection, CollectionRule, Note, Tag};
use chrono::Local;
use tauri::State;

#[tauri::command]
pub fn create_collection(
    state: State<'_, DbState>,
    name: String,
    rule_json: String,
) -> Result<Collection, String> {
    // Validate that rule_json is well-formed before saving.
    serde_json::from_str::<CollectionRule>(&rule_json)
        .map_err(|e| format!("Invalid rule JSON: {}", e))?;

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO collections (name, rule_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![name, rule_json, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Collection {
        id: conn.last_insert_rowid(),
        name,
        rule_json,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn get_collections(state: State<'_, DbState>) -> Result<Vec<Collection>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, rule_json, created_at, updated_at FROM collections ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
                rule_json: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
pub fn update_collection(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    rule_json: String,
) -> Result<Collection, String> {
    serde_json::from_str::<CollectionRule>(&rule_json)
        .map_err(|e| format!("Invalid rule JSON: {}", e))?;

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "UPDATE collections SET name = ?1, rule_json = ?2, updated_at = ?3 WHERE id = ?4",
        rusqlite::params![name, rule_json, now, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(Collection {
        id,
        name,
        rule_json,
        created_at: String::new(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn delete_collection(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM collections WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Execute a collection's rule and return matching notes.
#[tauri::command]
pub fn query_collection(
    state: State<'_, DbState>,
    id: i64,
) -> Result<Vec<Note>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // 1. Fetch the collection's rule_json
    let rule_json: String = conn
        .query_row(
            "SELECT rule_json FROM collections WHERE id = ?1",
            rusqlite::params![id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let rule: CollectionRule =
        serde_json::from_str(&rule_json).map_err(|e| format!("Invalid rule JSON: {}", e))?;

    // 2. Build a WHERE clause from the rule tree
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let where_clause = build_where_clause(&rule, &mut params);

    let sql = format!(
        "SELECT DISTINCT n.id, n.title, n.content, n.created_at, n.updated_at
         FROM notes n
         WHERE {}
         ORDER BY n.updated_at DESC",
        where_clause
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let notes: Vec<Note> = stmt
        .query_map(param_refs.as_slice(), |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                tags: vec![],
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // 3. Attach tags to each note
    let mut result = Vec::with_capacity(notes.len());
    for mut note in notes {
        note.tags = fetch_tags_for_note(&conn, note.id)?;
        result.push(note);
    }

    Ok(result)
}

// ── Helpers ──

/// Recursively build a SQL WHERE clause from a CollectionRule tree.
/// Each `Tag` leaf becomes a subquery checking note_tags.
fn build_where_clause(
    rule: &CollectionRule,
    params: &mut Vec<Box<dyn rusqlite::types::ToSql>>,
) -> String {
    match rule {
        CollectionRule::Tag { tag_id } => {
            params.push(Box::new(*tag_id));
            let idx = params.len(); // 1-indexed for SQLite ?N
            format!(
                "n.id IN (SELECT note_id FROM note_tags WHERE tag_id = ?{})",
                idx
            )
        }
        CollectionRule::And { children } => {
            if children.is_empty() {
                return "1".to_string();
            }
            let parts: Vec<String> = children
                .iter()
                .map(|c| build_where_clause(c, params))
                .collect();
            format!("({})", parts.join(" AND "))
        }
        CollectionRule::Or { children } => {
            if children.is_empty() {
                return "0".to_string();
            }
            let parts: Vec<String> = children
                .iter()
                .map(|c| build_where_clause(c, params))
                .collect();
            format!("({})", parts.join(" OR "))
        }
        CollectionRule::Not { child } => {
            let inner = build_where_clause(child, params);
            format!("NOT ({})", inner)
        }
    }
}

fn fetch_tags_for_note(
    conn: &rusqlite::Connection,
    note_id: i64,
) -> Result<Vec<Tag>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.name FROM tags t
             INNER JOIN note_tags nt ON nt.tag_id = t.id
             WHERE nt.note_id = ?1
             ORDER BY t.name",
        )
        .map_err(|e| e.to_string())?;

    let tags = stmt
        .query_map(rusqlite::params![note_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(tags)
}
