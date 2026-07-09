use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::bookmarks::{parse_chromium_bookmarks, ParsedBookmark};

pub struct AppState {
    pub db_path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportRequest {
    pub source_browser: String,
    pub source_path: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub imported_count: usize,
    pub active_count: i64,
    pub source_browser: String,
    pub source_path: String,
    pub report: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkRow {
    pub id: i64,
    pub source_browser: String,
    pub source_profile: String,
    pub source_path: String,
    pub folder_path: String,
    pub title: String,
    pub original_url: String,
    pub canonical_url: String,
    pub cleaned_url: String,
    pub date_added: Option<String>,
    pub status: String,
    pub archive_reason: Option<String>,
    pub link_check_state: String,
    pub last_checked_at: Option<String>,
    pub http_status: Option<i64>,
    pub check_attempts: i64,
    pub check_error: Option<String>,
    pub imported_at: String,
}

pub fn initialize_database(path: &Path) -> rusqlite::Result<()> {
    let connection = Connection::open(path)?;
    connection.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS imported_bookmarks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_browser TEXT NOT NULL,
            source_profile TEXT NOT NULL DEFAULT '',
            source_path TEXT NOT NULL,
            folder_path TEXT NOT NULL,
            title TEXT NOT NULL,
            original_url TEXT NOT NULL,
            canonical_url TEXT NOT NULL,
            cleaned_url TEXT NOT NULL,
            date_added TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            archive_reason TEXT,
            link_check_state TEXT NOT NULL DEFAULT 'unchecked',
            last_checked_at TEXT,
            http_status INTEGER,
            check_attempts INTEGER NOT NULL DEFAULT 0,
            check_error TEXT,
            imported_at TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_imported_bookmarks_status
            ON imported_bookmarks(status);

        CREATE INDEX IF NOT EXISTS idx_imported_bookmarks_canonical_url
            ON imported_bookmarks(canonical_url);

        CREATE TABLE IF NOT EXISTS import_reports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_browser TEXT NOT NULL,
            source_path TEXT NOT NULL,
            imported_count INTEGER NOT NULL,
            active_count INTEGER NOT NULL,
            report TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

#[tauri::command]
pub fn import_bookmarks_json(
    state: State<'_, AppState>,
    request: ImportRequest,
) -> Result<ImportResult, String> {
    let bookmarks = parse_chromium_bookmarks(&request.content)?;
    let imported_at = unix_timestamp();
    let mut connection = open_database(&state)?;
    let transaction = connection
        .transaction()
        .map_err(|err| format!("Could not start import transaction: {err}"))?;

    for bookmark in &bookmarks {
        insert_bookmark(
            &transaction,
            &request.source_browser,
            &request.source_path,
            bookmark,
            &imported_at,
        )
        .map_err(|err| format!("Could not insert bookmark '{}': {err}", bookmark.title))?;
    }

    let active_count: i64 = transaction
        .query_row(
            "SELECT COUNT(*) FROM imported_bookmarks WHERE status = 'active'",
            [],
            |row| row.get(0),
        )
        .map_err(|err| format!("Could not count active bookmarks: {err}"))?;

    let report = format!(
        "Import complete\nSource: {} ({})\nImported: {}\nActive pool: {}\nErrors: 0",
        request.source_browser,
        request.source_path,
        bookmarks.len(),
        active_count
    );

    transaction
        .execute(
            "
            INSERT INTO import_reports (
                source_browser,
                source_path,
                imported_count,
                active_count,
                report,
                created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
            params![
                request.source_browser,
                request.source_path,
                bookmarks.len() as i64,
                active_count,
                report,
                imported_at
            ],
        )
        .map_err(|err| format!("Could not save import report: {err}"))?;

    transaction
        .commit()
        .map_err(|err| format!("Could not commit import transaction: {err}"))?;

    Ok(ImportResult {
        imported_count: bookmarks.len(),
        active_count,
        source_browser: request.source_browser,
        source_path: request.source_path,
        report,
    })
}

#[tauri::command]
pub fn list_bookmarks(state: State<'_, AppState>) -> Result<Vec<BookmarkRow>, String> {
    let connection = open_database(&state)?;
    let mut statement = connection
        .prepare(
            "
            SELECT
                id,
                source_browser,
                source_profile,
                source_path,
                folder_path,
                title,
                original_url,
                canonical_url,
                cleaned_url,
                date_added,
                status,
                archive_reason,
                link_check_state,
                last_checked_at,
                http_status,
                check_attempts,
                check_error,
                imported_at
            FROM imported_bookmarks
            ORDER BY id DESC
            ",
        )
        .map_err(|err| format!("Could not prepare bookmark query: {err}"))?;

    let rows = statement
        .query_map([], |row| {
            Ok(BookmarkRow {
                id: row.get(0)?,
                source_browser: row.get(1)?,
                source_profile: row.get(2)?,
                source_path: row.get(3)?,
                folder_path: row.get(4)?,
                title: row.get(5)?,
                original_url: row.get(6)?,
                canonical_url: row.get(7)?,
                cleaned_url: row.get(8)?,
                date_added: row.get(9)?,
                status: row.get(10)?,
                archive_reason: row.get(11)?,
                link_check_state: row.get(12)?,
                last_checked_at: row.get(13)?,
                http_status: row.get(14)?,
                check_attempts: row.get(15)?,
                check_error: row.get(16)?,
                imported_at: row.get(17)?,
            })
        })
        .map_err(|err| format!("Could not query bookmarks: {err}"))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("Could not read bookmark row: {err}"))
}

fn open_database(state: &State<'_, AppState>) -> Result<Connection, String> {
    let connection = Connection::open(&state.db_path)
        .map_err(|err| format!("Could not open database: {err}"))?;
    initialize_database(&state.db_path)
        .map_err(|err| format!("Could not initialize database: {err}"))?;
    Ok(connection)
}

fn insert_bookmark(
    connection: &Connection,
    source_browser: &str,
    source_path: &str,
    bookmark: &ParsedBookmark,
    imported_at: &str,
) -> rusqlite::Result<usize> {
    connection.execute(
        "
        INSERT INTO imported_bookmarks (
            source_browser,
            source_path,
            folder_path,
            title,
            original_url,
            canonical_url,
            cleaned_url,
            date_added,
            imported_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        ",
        params![
            source_browser,
            source_path,
            bookmark.folder_path,
            bookmark.title,
            bookmark.original_url,
            bookmark.canonical_url,
            bookmark.cleaned_url,
            bookmark.date_added,
            imported_at
        ],
    )
}

fn unix_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

#[cfg(test)]
mod tests {
    use super::initialize_database;
    use rusqlite::Connection;

    #[test]
    fn initializes_bookmark_tables() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite should open");
        connection
            .execute_batch(
                "
                CREATE TABLE imported_bookmarks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    source_browser TEXT NOT NULL,
                    source_profile TEXT NOT NULL DEFAULT '',
                    source_path TEXT NOT NULL,
                    folder_path TEXT NOT NULL,
                    title TEXT NOT NULL,
                    original_url TEXT NOT NULL,
                    canonical_url TEXT NOT NULL,
                    cleaned_url TEXT NOT NULL,
                    date_added TEXT,
                    status TEXT NOT NULL DEFAULT 'active',
                    archive_reason TEXT,
                    link_check_state TEXT NOT NULL DEFAULT 'unchecked',
                    last_checked_at TEXT,
                    http_status INTEGER,
                    check_attempts INTEGER NOT NULL DEFAULT 0,
                    check_error TEXT,
                    imported_at TEXT NOT NULL
                );
                ",
            )
            .expect("schema should be valid");

        let count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table'",
                [],
                |row| row.get(0),
            )
            .expect("table count should be readable");
        assert!(count > 0);

        let temp_dir = tempfile::tempdir().expect("temp dir should exist");
        let db_path = temp_dir.path().join("test.sqlite3");
        initialize_database(&db_path).expect("database should initialize");
        assert!(db_path.exists());
    }
}
