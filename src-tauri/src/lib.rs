use tauri::{AppHandle, Manager};
use uuid::Uuid;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![create_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_entry(app: AppHandle, title: String, goals: Vec<String>, description: String) {
    println!("{}", format!("Creating entry with title: {}", title));
    let goalsVec: Vec<Goal> = goals.iter().map(|goal| Goal {title: goal.to_owned(), completed: false}).collect();
    save_entry_settings(app, title, goalsVec, description)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EntrySettings {
    title: String,
    goals: Vec<Goal>,
    description: String
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Goal {
    title: String,
    completed: bool
}

fn save_entry_settings(app: AppHandle, title: String, goals: Vec<Goal>, description: String) {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let id = Uuid::new_v4();
    println!("{}", format!("Saving entry with id: {}", id.to_string()));

    let entry_dir = app_data_dir.join(id.to_string());
    println!("{}", format!("Entry dir: {:?}", entry_dir));
    std::fs::create_dir_all(&entry_dir).expect("failed to create entry dir");
    
    let entry_settings = EntrySettings {title, goals, description};
    let entry_settings_path = entry_dir.join("settings.json");
    let entry_settings_json = serde_json::to_string_pretty(&entry_settings).expect("failed to serialize entry settings");
    std::fs::write(entry_settings_path, entry_settings_json).expect("failed to write entry settings");
}