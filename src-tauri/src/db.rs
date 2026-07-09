use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::BTreeMap;

use rusqlite::{params, Connection, Transaction};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::bookmarks::{clean_url, parse_chromium_bookmarks, ParsedBookmark};

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
pub struct CleanResult {
    pub active_count_before: i64,
    pub active_count_after: i64,
    pub duplicate_count: i64,
    pub query_cleaned_count: i64,
    pub removed_tracking_param_count: i64,
    pub archived_count: i64,
    pub dead_link_count: i64,
    pub errors: Vec<String>,
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

#[derive(Debug)]
struct ActiveBookmark {
    id: i64,
    original_url: String,
    canonical_url: String,
    cleaned_url: String,
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

        CREATE TABLE IF NOT EXISTS clean_reports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            active_count_before INTEGER NOT NULL,
            active_count_after INTEGER NOT NULL,
            duplicate_count INTEGER NOT NULL,
            query_cleaned_count INTEGER NOT NULL,
            removed_tracking_param_count INTEGER NOT NULL,
            archived_count INTEGER NOT NULL,
            dead_link_count INTEGER NOT NULL,
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
pub fn clean_bookmarks(state: State<'_, AppState>) -> Result<CleanResult, String> {
    let mut connection = open_database(&state)?;
    clean_database(&mut connection)
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
            ORDER BY
                CASE status WHEN 'active' THEN 0 ELSE 1 END,
                id DESC
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

pub fn clean_database(connection: &mut Connection) -> Result<CleanResult, String> {
    let cleaned_at = unix_timestamp();
    let transaction = connection
        .transaction()
        .map_err(|err| format!("Could not start clean transaction: {err}"))?;
    let active_bookmarks = load_active_bookmarks(&transaction)?;
    let active_count_before = active_bookmarks.len() as i64;
    let mut first_id_by_canonical_url = BTreeMap::<String, i64>::new();
    let mut duplicate_ids = Vec::<(i64, String)>::new();
    let mut query_cleaned_count = 0_i64;
    let mut removed_tracking_param_count = 0_i64;

    for bookmark in active_bookmarks {
        let clean_result = clean_url(&bookmark.original_url);
        let query_cleaned = !clean_result.removed_tracking_params.is_empty();

        if query_cleaned {
            query_cleaned_count += 1;
            removed_tracking_param_count += clean_result.removed_tracking_params.len() as i64;
        }

        if clean_result.cleaned_url != bookmark.cleaned_url
            || clean_result.cleaned_url != bookmark.canonical_url
        {
            transaction
                .execute(
                    "
                    UPDATE imported_bookmarks
                    SET canonical_url = ?1,
                        cleaned_url = ?1
                    WHERE id = ?2
                    ",
                    params![clean_result.cleaned_url, bookmark.id],
                )
                .map_err(|err| format!("Could not update cleaned URL: {err}"))?;
        }

        if first_id_by_canonical_url.contains_key(&clean_result.cleaned_url) {
            let archive_reason = if query_cleaned {
                "query_cleaned_duplicate"
            } else {
                "duplicate"
            };
            duplicate_ids.push((bookmark.id, archive_reason.to_string()));
        } else {
            first_id_by_canonical_url.insert(clean_result.cleaned_url.clone(), bookmark.id);
        }
    }

    for (bookmark_id, archive_reason) in &duplicate_ids {
        transaction
            .execute(
                "
                UPDATE imported_bookmarks
                SET status = 'archived',
                    archive_reason = ?1
                WHERE id = ?2
                ",
                params![archive_reason, bookmark_id],
            )
            .map_err(|err| format!("Could not archive duplicate bookmark: {err}"))?;
    }

    let active_count_after = count_active_bookmarks(&transaction)?;
    let duplicate_count = duplicate_ids.len() as i64;
    let archived_count = duplicate_count;
    let dead_link_count = 0_i64;
    let errors = Vec::<String>::new();
    let report = format!(
        "Clean complete\nActive before: {active_count_before}\nActive after: {active_count_after}\nTracking-query cleaned: {query_cleaned_count}\nTracking parameters removed: {removed_tracking_param_count}\nDuplicates archived: {duplicate_count}\nDead links archived: {dead_link_count} (not checked in Slice 0.2)\nErrors: 0"
    );

    transaction
        .execute(
            "
            INSERT INTO clean_reports (
                active_count_before,
                active_count_after,
                duplicate_count,
                query_cleaned_count,
                removed_tracking_param_count,
                archived_count,
                dead_link_count,
                report,
                created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ",
            params![
                active_count_before,
                active_count_after,
                duplicate_count,
                query_cleaned_count,
                removed_tracking_param_count,
                archived_count,
                dead_link_count,
                report,
                cleaned_at
            ],
        )
        .map_err(|err| format!("Could not save clean report: {err}"))?;

    transaction
        .commit()
        .map_err(|err| format!("Could not commit clean transaction: {err}"))?;

    Ok(CleanResult {
        active_count_before,
        active_count_after,
        duplicate_count,
        query_cleaned_count,
        removed_tracking_param_count,
        archived_count,
        dead_link_count,
        errors,
        report,
    })
}

fn open_database(state: &State<'_, AppState>) -> Result<Connection, String> {
    let connection = Connection::open(&state.db_path)
        .map_err(|err| format!("Could not open database: {err}"))?;
    initialize_database(&state.db_path)
        .map_err(|err| format!("Could not initialize database: {err}"))?;
    Ok(connection)
}

fn load_active_bookmarks(transaction: &Transaction<'_>) -> Result<Vec<ActiveBookmark>, String> {
    let mut statement = transaction
        .prepare(
            "
            SELECT id, original_url, canonical_url, cleaned_url
            FROM imported_bookmarks
            WHERE status = 'active'
            ORDER BY id ASC
            ",
        )
        .map_err(|err| format!("Could not prepare active bookmark query: {err}"))?;

    let rows = statement
        .query_map([], |row| {
            Ok(ActiveBookmark {
                id: row.get(0)?,
                original_url: row.get(1)?,
                canonical_url: row.get(2)?,
                cleaned_url: row.get(3)?,
            })
        })
        .map_err(|err| format!("Could not query active bookmarks: {err}"))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("Could not read active bookmark row: {err}"))
}

fn count_active_bookmarks(transaction: &Transaction<'_>) -> Result<i64, String> {
    transaction
        .query_row(
            "SELECT COUNT(*) FROM imported_bookmarks WHERE status = 'active'",
            [],
            |row| row.get(0),
        )
        .map_err(|err| format!("Could not count active bookmarks: {err}"))
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
    use super::{clean_database, initialize_database, insert_bookmark};
    use crate::bookmarks::ParsedBookmark;
    use rusqlite::Connection;

    #[test]
    fn initializes_bookmark_tables() {
        let temp_dir = tempfile::tempdir().expect("temp dir should exist");
        let db_path = temp_dir.path().join("test.sqlite3");
        initialize_database(&db_path).expect("database should initialize");
        assert!(db_path.exists());

        let connection = Connection::open(db_path).expect("database should open");
        let count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('imported_bookmarks', 'import_reports', 'clean_reports')",
                [],
                |row| row.get(0),
            )
            .expect("table count should be readable");
        assert_eq!(count, 3);
    }

    #[test]
    fn clean_database_removes_tracking_params_and_archives_duplicates() {
        let temp_dir = tempfile::tempdir().expect("temp dir should exist");
        let db_path = temp_dir.path().join("clean.sqlite3");
        initialize_database(&db_path).expect("database should initialize");
        let mut connection = Connection::open(db_path).expect("database should open");
        let imported_at = "1";

        let first = ParsedBookmark {
            folder_path: "Bookmarks Bar".to_string(),
            title: "Tracked".to_string(),
            original_url: "https://example.com/page?id=7&utm_source=email".to_string(),
            canonical_url: "https://example.com/page?id=7&utm_source=email".to_string(),
            cleaned_url: "https://example.com/page?id=7&utm_source=email".to_string(),
            date_added: None,
        };
        let second = ParsedBookmark {
            folder_path: "Bookmarks Bar".to_string(),
            title: "Clean".to_string(),
            original_url: "https://example.com/page?id=7".to_string(),
            canonical_url: "https://example.com/page?id=7".to_string(),
            cleaned_url: "https://example.com/page?id=7".to_string(),
            date_added: None,
        };

        insert_bookmark(&connection, "chrome", "fixture", &first, imported_at)
            .expect("first bookmark should insert");
        insert_bookmark(&connection, "edge", "fixture", &second, imported_at)
            .expect("second bookmark should insert");

        let result = clean_database(&mut connection).expect("clean should succeed");

        assert_eq!(result.active_count_before, 2);
        assert_eq!(result.active_count_after, 1);
        assert_eq!(result.query_cleaned_count, 1);
        assert_eq!(result.removed_tracking_param_count, 1);
        assert_eq!(result.duplicate_count, 1);

        let archived_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM imported_bookmarks WHERE status = 'archived'",
                [],
                |row| row.get(0),
            )
            .expect("archived count should be readable");
        assert_eq!(archived_count, 1);
    }
}
