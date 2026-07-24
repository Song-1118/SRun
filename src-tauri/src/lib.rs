pub mod modules;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      // Initialize file storage
      modules::database::connection::init_storage(app.handle())
        .expect("Failed to initialize storage");
      
      // Initialize data seeder (first time only)
      modules::database::seeder::ensure_seeded().ok();
      
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
