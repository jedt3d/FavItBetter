mod bookmarks;
mod db;

use std::error::Error;
use std::fs;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| setup_app(app))
        .invoke_handler(tauri::generate_handler![
            db::clean_bookmarks,
            db::export_report_txt,
            db::import_bookmarks_json,
            db::list_bookmarks
        ])
        .run(tauri::generate_context!())
        .expect("error while running FavItBetter");
}

fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    let app_data_dir = app.path().app_data_dir()?;
    fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("favitbetter.sqlite3");
    db::initialize_database(&db_path)?;
    app.manage(db::AppState { db_path });

    Ok(())
}
