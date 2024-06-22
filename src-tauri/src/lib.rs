use tauri::{AppHandle, Manager};
use uuid::Uuid;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![create_entry, read_entries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_entry(app: AppHandle, title: String, goals: Vec<String>, description: String) {
    println!("{}", format!("Creating entry with title: {}", title));
    let goals_vec: Vec<Goal> = goals.iter().map(|goal| Goal {title: goal.to_owned(), completed: false}).collect();
    save_entry_settings(app, title, goals_vec, description)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EntrySettings {
    title: String,
    goals: Vec<Goal>,
    content: Vec<ContentPiece>,
    description: String
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Goal {
    title: String,
    completed: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
struct ContentPiece {
    r#type: String,
    path: Option<String>,
    text: Option<String>,
    creation_date: i32
}

fn save_entry_settings(app: AppHandle, title: String, goals: Vec<Goal>, description: String) {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let entries_dir = app_data_dir.join("entries");
    if !entries_dir.exists() {std::fs::create_dir_all(&entries_dir).expect("failed to create entries dir");}

    let id = Uuid::new_v4();
    println!("{}", format!("Saving entry with id: {}", id.to_string()));

    let entry_dir = entries_dir.join(id.to_string());
    println!("{}", format!("Entry dir: {:?}", entry_dir));
    std::fs::create_dir_all(&entry_dir).expect("failed to create entry dir");
    
    let entry_settings = EntrySettings {title, goals, content: Vec::new(), description};
    let entry_settings_path = entry_dir.join("settings.json");
    let entry_settings_json = serde_json::to_string_pretty(&entry_settings).expect("failed to serialize entry settings");
    std::fs::write(entry_settings_path, entry_settings_json).expect("failed to write entry settings");
}

#[tauri::command]
fn read_entries(app: AppHandle) -> Vec<EntrySettings> {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let entries_dir = app_data_dir.join("entries");
    if !entries_dir.exists() {std::fs::create_dir_all(&entries_dir).expect("failed to create entries dir");}

    std::fs::read_dir(entries_dir).expect("couldn't read entries")
    .filter(|entry| {entry.as_ref().unwrap().path().is_dir()})
    .map(|entry| {
        let entry = entry.expect("failed to read entry");
        let entry_settings_path = entry.path().join("settings.json");
        let entry_settings_json: EntrySettings = serde_json::from_str(&std::fs::read_to_string(entry_settings_path).expect("failed to read entry settings")).expect("failed to deserialize entry settings");
        return entry_settings_json;
    }).collect()
}