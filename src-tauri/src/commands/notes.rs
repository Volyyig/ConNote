use crate::db::DbState;
use crate::models::{Note, Tag};
use chrono::Local;
use tauri::State;

#[tauri::command]
pub fn create_note(
    state: State<'_, DbState>,
    title: String,
    content: String,
    tag_ids: Vec<i64>,
) -> Result<Note, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO notes (title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![title, content, now, now],
    )
    .map_err(|e| e.to_string())?;

    let note_id = conn.last_insert_rowid();

    // Link tags
    for tag_id in &tag_ids {
        conn.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            rusqlite::params![note_id, tag_id],
        )
        .map_err(|e| e.to_string())?;
    }

    // Fetch the tags to return them with the note
    let tags = fetch_tags_for_note(&conn, note_id)?;

    Ok(Note {
        id: note_id,
        title,
        content,
        created_at: now.clone(),
        updated_at: now,
        tags,
    })
}

#[tauri::command]
pub fn get_notes(state: State<'_, DbState>) -> Result<Vec<Note>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, title, content, created_at, updated_at FROM notes ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let notes: Vec<Note> = stmt
        .query_map([], |row| {
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

    // Attach tags to each note
    let mut result = Vec::with_capacity(notes.len());
    for mut note in notes {
        note.tags = fetch_tags_for_note(&conn, note.id)?;
        result.push(note);
    }

    Ok(result)
}

#[tauri::command]
pub fn get_note(state: State<'_, DbState>, id: i64) -> Result<Note, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut note: Note = conn
        .query_row(
            "SELECT id, title, content, created_at, updated_at FROM notes WHERE id = ?1",
            rusqlite::params![id],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    tags: vec![],
                })
            },
        )
        .map_err(|e| e.to_string())?;

    note.tags = fetch_tags_for_note(&conn, note.id)?;
    Ok(note)
}

#[tauri::command]
pub fn update_note(
    state: State<'_, DbState>,
    id: i64,
    title: String,
    content: String,
    tag_ids: Vec<i64>,
) -> Result<Note, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        rusqlite::params![title, content, now, id],
    )
    .map_err(|e| e.to_string())?;

    // Re-sync tags: delete all, then re-insert
    conn.execute(
        "DELETE FROM note_tags WHERE note_id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;

    for tag_id in &tag_ids {
        conn.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            rusqlite::params![id, tag_id],
        )
        .map_err(|e| e.to_string())?;
    }

    let tags = fetch_tags_for_note(&conn, id)?;

    Ok(Note {
        id,
        title,
        content,
        created_at: String::new(), // caller already has this
        updated_at: now,
        tags,
    })
}

#[tauri::command]
pub fn delete_note(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM notes WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ── Helper ──

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
