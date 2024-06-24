use std::{fs, path::PathBuf, time::{self, SystemTime, UNIX_EPOCH}};

use tauri::{AppHandle, Event, Manager};
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};
use uuid::Uuid;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![create_entry, read_entries, get_settings, change_content_piece, add_content_piece])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_entry(app: AppHandle, title: String, goals: Vec<String>, description: String) {
    let goals_vec: Vec<Goal> = goals.iter().map(|goal| Goal {title: goal.to_owned(), completed: false}).collect();
    save_entry_settings(app, title, goals_vec, description)
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct EntrySettings {
    uuid: String,
    title: String,
    goals: Vec<Goal>,
    content: Vec<ContentPiece>,
    description: String
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Goal {
    title: String,
    completed: bool
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct ContentPiece {
    r#type: String,
    path: Option<String>,
    text: Option<String>,
    creation_date: u64
}

fn save_entry_settings(app: AppHandle, title: String, goals: Vec<Goal>, description: String) {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let entries_dir = app_data_dir.join("entries");
    if !entries_dir.exists() {std::fs::create_dir_all(&entries_dir).expect("failed to create entries dir");}

    let id = Uuid::new_v4();

    let entry_dir = entries_dir.join(id.to_string());
    std::fs::create_dir_all(&entry_dir).expect("failed to create entry dir");

    let content_dir = entry_dir.join("content");
    if !content_dir.exists() {std::fs::create_dir_all(&content_dir).expect("failed to create content dir");}
    
    let entry_settings = EntrySettings {uuid: id.to_string(), title, goals, content: Vec::new(), description};
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

#[tauri::command]
fn get_settings(app: AppHandle, uuid: String) -> EntrySettings {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let entries_dir = app_data_dir.join("entries");
    if !entries_dir.exists() {std::fs::create_dir_all(&entries_dir).expect("failed to create entries dir");}

    let entry_dir = entries_dir.join(uuid);
    let entry_settings_path = entry_dir.join("settings.json");
    let entry_settings_json: EntrySettings = serde_json::from_str(&std::fs::read_to_string(entry_settings_path).expect("failed to read entry settings")).expect("failed to deserialize entry settings");
    return entry_settings_json;
}

#[tauri::command]
fn change_content_piece(app: AppHandle, uuid: String, index: usize, text: Option<String>) {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let entries_dir = app_data_dir.join("entries");
    if entries_dir.exists() {
        let entry_dir = entries_dir.join(&uuid);
        if entry_dir.exists() {
            let settings_path = entry_dir.join("settings.json");
            let mut settings_json: EntrySettings = serde_json::from_str(&std::fs::read_to_string(&settings_path).expect("failed to read entry settings")).expect("failed to deserialize entry settings");
            let mut content_piece: ContentPiece = settings_json.content[index].clone();

            if content_piece.r#type != "text" {
                let mut dialog = DialogExt::dialog(&app).file();
                match content_piece.r#type.as_str() {
                    "image"=>dialog = dialog.add_filter("Image", &["png", "jpg", "jpeg"]),
                    "video"=>dialog = dialog.add_filter("Video", &["mp4", "mov", "mkv", "webm"]),
                    "audio"=>dialog = dialog.add_filter("Audio", &["mp3", "wav", "ogg"]),
                    _=>()
                }
                dialog.pick_file(move |file_path| {
                    if file_path.is_some() {
                        let content_path = file_path.unwrap().path;
                        let content_uuid = Uuid::new_v4();
                        let new_file_name = format!("{}.{}", content_uuid.to_string(), PathBuf::from(&content_path).extension().unwrap().to_str().unwrap());
                        let new_content_path = entry_dir.join("content").join(&new_file_name);
                        fs::copy(content_path, &new_content_path).expect("failed to copy content file");
                        let old_content_path = entry_dir.join("content").join(settings_json.content[index].path.as_ref().unwrap());
                        if old_content_path.exists() {fs::remove_file(&old_content_path).expect("failed to remove old content file");}

                        content_piece.path = Some(new_file_name);
                        settings_json.content[index] = content_piece;
                        fs::write(settings_path, serde_json::to_string_pretty(&settings_json).expect("failed to serialize entry settings")).expect("failed to write entry settings");
                        app.emit("reload_entry", uuid).unwrap();
                    }
                });
            }else {
                content_piece.text = text;
                settings_json.content[index] = content_piece;
                fs::write(settings_path, serde_json::to_string_pretty(&settings_json).expect("failed to serialize entry settings")).expect("failed to write entry settings");
                app.emit("reload_entry", uuid).unwrap();
            }
        }
    }
}

#[tauri::command]
fn add_content_piece(app: AppHandle, uuid: String, r#type: String) {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let entries_dir = app_data_dir.join("entries");
    if entries_dir.exists() {
        let entry_dir = entries_dir.join(&uuid);
        if entry_dir.exists() {
            let settings_path = entry_dir.join("settings.json");
            let mut settings_json: EntrySettings = serde_json::from_str(&std::fs::read_to_string(&settings_path).expect("failed to read entry settings")).expect("failed to deserialize entry settings");

            if r#type != "text" {
                let mut dialog = DialogExt::dialog(&app).file();
                match r#type.as_str() {
                    "image"=>dialog = dialog.add_filter("Image", &["png", "jpg", "jpeg"]),
                    "video"=>dialog = dialog.add_filter("Video", &["mp4", "mov", "mkv", "webm"]),
                    "audio"=>dialog = dialog.add_filter("Audio", &["mp3", "wav", "ogg"]),
                    _=>()
                }
                dialog.pick_file(move |file_path| {
                    if file_path.is_some() {
                        let content_path = file_path.unwrap().path;
                        let content_uuid = Uuid::new_v4();
                        let new_file_name = format!("{}.{}", content_uuid.to_string(), PathBuf::from(&content_path).extension().unwrap().to_str().unwrap());
                        let new_content_path = entry_dir.join("content").join(&new_file_name);
                        fs::copy(content_path, &new_content_path).expect("failed to copy content file");

                        settings_json.content.push(ContentPiece {r#type, path: Some(new_file_name), text: None, creation_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()});
                        fs::write(settings_path, serde_json::to_string_pretty(&settings_json).expect("failed to serialize entry settings")).expect("failed to write entry settings");
                        app.emit("reload_entry", uuid).unwrap();
                    }
                });
            }else {
                settings_json.content.push(ContentPiece {r#type, path: None, text: Some("".to_owned()), creation_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()});
                fs::write(settings_path, serde_json::to_string_pretty(&settings_json).expect("failed to serialize entry settings")).expect("failed to write entry settings");
                app.emit("reload_entry", uuid).unwrap();
            }
        }
    }
}