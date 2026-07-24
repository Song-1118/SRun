use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

static DATA_DIR: OnceCell<PathBuf> = OnceCell::new();

pub fn init_storage(app_handle: &AppHandle) -> Result<(), String> {
    let app_data = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    std::fs::create_dir_all(&app_data)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    
    DATA_DIR.set(app_data).ok();
    
    // Create subdirectories
    let games_dir = get_games_dir();
    let history_dir = get_history_dir();
    std::fs::create_dir_all(&games_dir)
        .map_err(|e| format!("Failed to create games directory: {}", e))?;
    std::fs::create_dir_all(&history_dir)
        .map_err(|e| format!("Failed to create history directory: {}", e))?;
    
    Ok(())
}

pub fn get_data_dir() -> &'static PathBuf {
    DATA_DIR.get().expect("Data directory not initialized")
}

pub fn get_games_dir() -> PathBuf {
    get_data_dir().join("games")
}

pub fn get_history_dir() -> PathBuf {
    get_data_dir().join("history")
}

pub fn get_games_file() -> PathBuf {
    get_games_dir().join("games.json")
}

pub fn get_requirements_file() -> PathBuf {
    get_games_dir().join("requirements.json")
}

pub fn get_history_file() -> PathBuf {
    get_history_dir().join("history.json")
}
