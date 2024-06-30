use std::{fs::{self, File}, io::{Read, Write}, path::PathBuf};

use chrono::Local;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use uuid::Uuid;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![create_entry, read_entries, get_settings, change_content_piece, add_content_piece, remove_content_piece, change_goal_state, export_entries, import_entries])
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
    creation_date: i64
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

                        settings_json.content.push(ContentPiece {r#type, path: Some(new_file_name), text: None, creation_date: Local::now().timestamp()});
                        fs::write(settings_path, serde_json::to_string_pretty(&settings_json).expect("failed to serialize entry settings")).expect("failed to write entry settings");
                        app.emit("reload_entry", uuid).unwrap();
                    }
                });
            }else {
                settings_json.content.push(ContentPiece {r#type, path: None, text: Some("".to_owned()), creation_date: Local::now().timestamp()});
                fs::write(settings_path, serde_json::to_string_pretty(&settings_json).expect("failed to serialize entry settings")).expect("failed to write entry settings");
                app.emit("reload_entry", uuid).unwrap();
            }
        }
    }
}

#[tauri::command]
fn remove_content_piece(app: AppHandle, uuid: String, index: usize) {
    DialogExt::dialog(&app)
    .message("Are you sure you want to remove this content piece? This action cannot be reverted.")
    .cancel_button_label("Cancel")
    .ok_button_label("Continue")
    .title("Confirm Removal")
    .kind(MessageDialogKind::Info)
    .show(move |response| {
        if response == true {
            let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            let entries_dir = app_data_dir.join("entries");
            if entries_dir.exists() {
                let entry_dir = entries_dir.join(&uuid);
                if entry_dir.exists() {
                    let settings_path = entry_dir.join("settings.json");
                    let mut settings_json: EntrySettings = serde_json::from_str(&std::fs::read_to_string(&settings_path).expect("failed to read entry settings")).expect("failed to deserialize entry settings");
                    let content_piece = settings_json.content.remove(index);
                    if content_piece.r#type != "text" {
                        let content_path = entry_dir.join("content").join(content_piece.path.unwrap());
                        if content_path.exists() {fs::remove_file(&content_path).expect("failed to remove content file");}
                    }
                    fs::write(settings_path, serde_json::to_string_pretty(&settings_json).expect("failed to serialize entry settings")).expect("failed to write entry settings");
                }
            }
            app.emit("reload_entry", uuid).unwrap();
        }
    });
}

#[tauri::command]
fn change_goal_state(app: AppHandle, uuid: String, index: usize, completed: bool) {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let entries_dir = app_data_dir.join("entries");
    if entries_dir.exists() {
        let entry_dir = entries_dir.join(&uuid);
        if entry_dir.exists() {
            let settings_path = entry_dir.join("settings.json");
            let mut settings_json: EntrySettings = serde_json::from_str(&std::fs::read_to_string(&settings_path).expect("failed to read entry settings")).expect("failed to deserialize entry settings");
            settings_json.goals[index].completed = completed;
            fs::write(settings_path, serde_json::to_string_pretty(&settings_json).expect("failed to serialize entry settings")).expect("failed to write entry settings");
            app.emit("reload_entry", uuid).unwrap();
        }
    }
}


#[tauri::command]
fn export_entries(app: AppHandle) {
    println!("exporting entries");
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    let entries_dir = app_data_dir.join("entries");
    let file_name = format!("archivement_{}.zip", Local::now().format("%Y-%m-%d_%H%M%S"));
    if entries_dir.exists() {
        DialogExt::dialog(&app)
        .file()
        .set_title("Select a destination to export your entries")
        .set_can_create_directories(true)
        .set_file_name(file_name)
        .save_file(move |path_result| {
            if path_result.is_some() {
                let zip_path = path_result.unwrap();
                let file = File::create(&zip_path).unwrap();
    
                let walkdir = WalkDir::new(&entries_dir).into_iter();
        
                let mut zip = zip::ZipWriter::new(file);
        
                let options = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o755);

                let mut buffer: Vec<u8> = Vec::new();
                for entry in walkdir {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    let name = path.strip_prefix(&entries_dir).unwrap();
                    if path.is_file(){
                        zip.start_file(name.to_str().unwrap().replace("\\", "/"), options)
                            .unwrap();
                        let mut f = File::open(path).unwrap();
        
                        f.read_to_end(&mut buffer).unwrap();
                        zip.write_all(&*buffer).unwrap();
                        buffer.clear();
                    } else if path.is_dir() && name.as_os_str().len() != 0 {
                        zip.add_directory(name.to_str().unwrap().replace("\\", "/"), options).unwrap();
                    }
                }
                zip.finish().unwrap();
            }
        })
    }
}

#[tauri::command]
fn import_entries(app: AppHandle) {
    DialogExt::dialog(&app)
    .file()
    .add_filter("Zip Archive", &["zip"])
    .pick_file(move |path_result| {
        if path_result.is_some() {
            let zip_path = path_result.unwrap().path;
            let file = File::open(&zip_path).unwrap();
            let mut archive = zip::ZipArchive::new(file).unwrap();
            let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            let entries_dir = app_data_dir.join("entries");
            if !entries_dir.exists() {std::fs::create_dir_all(&entries_dir).expect("failed to create entries dir");}
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let file_name = file.enclosed_name().unwrap();
                let file_path = entries_dir.join(file_name);
                println!("extracting file: {}", file_path.to_str().unwrap());
                if file.is_dir() {
                    std::fs::create_dir_all(&file_path).expect("failed to create entry dir");
                }else {
                    let mut new_file = File::create(&file_path).unwrap();
                    std::io::copy(&mut file, &mut new_file).unwrap();
                }
            }
        }
    });
}