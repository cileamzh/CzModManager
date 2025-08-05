use std::{
    fs::{
        self, copy, create_dir, create_dir_all, read_dir, remove_dir_all, rename, File, OpenOptions,
    },
    io::{self, ErrorKind, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::{LazyLock, RwLock},
    thread,
};

use actix_files::Files;
use actix_web::{dev::Server, HttpServer};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string, Value};
use sevenz_rust::decompress_file;
use tauri::{AppHandle, Emitter, Manager};

static FOLDER: &'static str = "../res";
static IS_RUN: LazyLock<RwLock<bool>> = LazyLock::new(|| RwLock::new(false));

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // 在新实例启动时，让主窗口获得焦点
            let window = app.get_webview_window("main").unwrap();
            window.set_focus().unwrap();
        }))
        .invoke_handler(tauri::generate_handler![
            save_games,
            get_games,
            check_mods_folder,
            run_game,
            add_mod,
            get_mods,
            disable_mod,
            enable_mod,
            delete_mod,
            get_previews,
            get_json_file,
            set_json_file,
            add_icon,
            start_server,
            add_class_icon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub(crate) fn build_server(port: u16) -> Server {
    HttpServer::new(move || {
        actix_web::App::new().service(Files::new("/", FOLDER).show_files_listing())
    })
    .bind(("127.0.0.1", port))
    .unwrap()
    .run()
}

#[tauri::command]
async fn start_server(app: AppHandle) -> String {
    app.emit("log", "start_server").unwrap();
    let mut is_run = IS_RUN.write().unwrap();
    if *is_run {
        app.emit("log", "Server is already running.").unwrap();
        return "Running".to_string();
    }
    *is_run = true;
    tokio::spawn(build_server(12981));
    app.emit("log", "Server started successfully.").unwrap();
    return "ok".to_string();
}

#[tauri::command]
fn get_json_file(app: AppHandle, name: String) -> Value {
    let path = Path::new(FOLDER);
    app.emit(
        "log",
        format!(
            "Attempting to read JSON file from: {}",
            path.to_string_lossy()
        ),
    )
    .unwrap();
    if let Ok(mut file) = OpenOptions::new().read(true).open(path.join(&name)) {
        let mut rstr = String::new();
        match file.read_to_string(&mut rstr) {
            Ok(_) => {
                app.emit("log", format!("Successfully read file: {}", name))
                    .unwrap();
                match from_str(&rstr) {
                    Ok(val) => val,
                    Err(e) => {
                        app.emit(
                            "log",
                            format!("Failed to parse JSON from file {}: {}", name, e),
                        )
                        .unwrap();
                        from_str("{}").unwrap()
                    }
                }
            }
            Err(e) => {
                app.emit("log", format!("Failed to read file {}: {}", name, e))
                    .unwrap();
                from_str("{}").unwrap()
            }
        }
    } else {
        app.emit("log", format!("Failed to open file: {}", name))
            .unwrap();
        from_str("{}").unwrap()
    }
}

#[tauri::command]
fn set_json_file(app: AppHandle, name: String, data: Value) {
    let path = Path::new(FOLDER).join(&name);
    app.emit(
        "log",
        format!(
            "Attempting to write JSON to file: {}",
            path.to_string_lossy()
        ),
    )
    .unwrap();
    if let Ok(mut file) = OpenOptions::new().write(true).open(&path) {
        match file.set_len(0) {
            Ok(_) => match file.seek(SeekFrom::Start(0)) {
                Ok(_) => match to_string(&data) {
                    Ok(json_str) => match file.write_all(json_str.as_bytes()) {
                        Ok(_) => app
                            .emit("log", format!("Successfully wrote to file: {}", name))
                            .unwrap(),
                        Err(e) => app
                            .emit(
                                "log",
                                format!("Failed to write data to file {}: {}", name, e),
                            )
                            .unwrap(),
                    },
                    Err(e) => app
                        .emit(
                            "log",
                            format!("Failed to serialize JSON data for file {}: {}", name, e),
                        )
                        .unwrap(),
                },
                Err(e) => app
                    .emit(
                        "log",
                        format!("Failed to seek to start of file {}: {}", name, e),
                    )
                    .unwrap(),
            },
            Err(e) => app
                .emit("log", format!("Failed to truncate file {}: {}", name, e))
                .unwrap(),
        }
    } else {
        app.emit("log", format!("Failed to open file for writing: {}", name))
            .unwrap();
    }
}

#[tauri::command]
fn check_mods_folder(app: AppHandle, path: PathBuf) -> String {
    if !path.is_dir() {
        app.emit(
            "log",
            format!("Path isn't a directory: {}", path.to_string_lossy()),
        )
        .unwrap();
        return "err Path isn't a dir".to_string();
    }
    match path.read_dir() {
        Ok(dir) => {
            for entry in dir {
                if let Ok(entry) = entry {
                    if entry.file_name() == "3DMigoto Loader.exe" {
                        app.emit("log", "Found '3DMigoto Loader.exe'.").unwrap();
                        return "ok".to_string();
                    }
                } else if let Err(e) = entry {
                    app.emit("log", format!("Error reading directory entry: {}", e))
                        .unwrap();
                }
            }
        }
        Err(e) => {
            app.emit("log", format!("Failed to read directory: {}", e))
                .unwrap();
        }
    }
    app.emit("log", "Did not find '3DMigoto Loader.exe'.")
        .unwrap();
    "err Doesn't Find 3DMigoto Loader.exe".to_string()
}

#[tauri::command]
async fn add_icon(app: AppHandle, game: String, class: String, icon: PathBuf) -> Value {
    if !icon.is_file() {
        app.emit("log", "Provided path is not a file.").unwrap();
        return json!({"msg":"err"});
    }

    let ext = match icon.extension() {
        Some(ext) => ext.to_string_lossy().to_string(),
        None => "".to_string(),
    };
    let fname = format!("{}.{}", nanoid!(8).to_string(), ext);
    let target_dir = Path::new(FOLDER).join("img").join(&game).join(&class);
    let file = target_dir.join(&fname);

    // 确保目标目录存在
    if let Err(e) = create_dir_all(&target_dir) {
        app.emit(
            "log",
            format!("Failed to create directory {:?}: {}", target_dir, e),
        )
        .unwrap();
        return json!({"msg":"err"});
    }

    if let Err(e) = copy(&icon, &file) {
        app.emit(
            "log",
            format!("Failed to copy icon from {:?} to {:?}: {}", icon, file, e),
        )
        .unwrap();
        return json!({"msg":"err"});
    }
    app.emit("log", format!("Successfully copied icon to: {:?}", file))
        .unwrap();
    json!({"msg":"ok","file_name":fname})
}

#[tauri::command]
async fn add_class_icon(app: AppHandle, game: String, class: String, icon: PathBuf) -> Value {
    if !icon.is_file() {
        app.emit("log", "Provided path is not a file.").unwrap();
        return json!({"msg":"err"});
    }

    let ext = match icon.extension() {
        Some(ext) => ext.to_string_lossy().to_string(),
        None => "".to_string(),
    };
    let fname = format!("{}.{}", nanoid!(8).to_string(), ext);
    let target_dir = Path::new(FOLDER).join("img").join(&game).join(&class);
    let file = target_dir.join(&fname);

    // 确保目标目录存在
    if let Err(e) = create_dir_all(&target_dir) {
        app.emit(
            "log",
            format!("Failed to create directory {:?}: {}", target_dir, e),
        )
        .unwrap();
        return json!({"msg":"err"});
    }

    if let Err(e) = copy(&icon, &file) {
        app.emit(
            "log",
            format!("Failed to copy icon from {:?} to {:?}: {}", icon, file, e),
        )
        .unwrap();
        return json!({"msg":"err"});
    }
    app.emit("log", format!("Successfully copied icon to: {:?}", file))
        .unwrap();
    json!({"msg":"ok","file_name":format!("/img/{}/{}/{}",game,class,fname)})
}

#[tauri::command]
fn get_games(app: AppHandle) -> Vec<Game> {
    let path = Path::new(FOLDER);
    app.emit(
        "log",
        format!("Attempting to read games from: {}", path.to_string_lossy()),
    )
    .unwrap();
    if let Ok(mut file) = OpenOptions::new().read(true).open(&path.join("games.json")) {
        let mut rstr = String::new();
        if let Err(e) = file.read_to_string(&mut rstr) {
            app.emit("log", format!("Failed to read games.json: {}", e))
                .unwrap();
            return Vec::new();
        }
        match from_str(&rstr) {
            Ok(games) => {
                app.emit("log", "Successfully retrieved games list.")
                    .unwrap();
                games
            }
            Err(e) => {
                app.emit("log", format!("Failed to parse games.json: {}", e))
                    .unwrap();
                Vec::new()
            }
        }
    } else {
        app.emit("log", "Failed to open games.json.".to_string())
            .unwrap();
        Vec::new()
    }
}

#[tauri::command]
fn save_games(app: AppHandle, games: Vec<Game>) {
    let path = format!("{}/games.json", FOLDER);
    app.emit("log", format!("Attempting to save games to: {}", path))
        .unwrap();
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(&path) {
        match to_string(&games) {
            Ok(data) => {
                if let Err(e) = file.set_len(0) {
                    app.emit("log", format!("Failed to truncate games.json: {}", e))
                        .unwrap();
                    return;
                }
                if let Err(e) = file.seek(SeekFrom::Start(0)) {
                    app.emit("log", format!("Failed to seek in games.json: {}", e))
                        .unwrap();
                    return;
                }
                if let Err(e) = file.write_all(data.as_bytes()) {
                    app.emit("log", format!("Failed to write to games.json: {}", e))
                        .unwrap();
                    return;
                }
                app.emit("log", "Successfully saved games.".to_string())
                    .unwrap();
            }
            Err(e) => {
                app.emit("log", format!("Failed to serialize games data: {}", e))
                    .unwrap();
            }
        }
    } else {
        app.emit("log", "Failed to open games.json for writing.".to_string())
            .unwrap();
    }
}

#[tauri::command]
fn run_game(app: AppHandle, migoto_path: PathBuf, game_path: Option<PathBuf>) -> bool {
    let migoto_exe = migoto_path.join("3DMigoto Loader.exe");
    app.emit(
        "log",
        format!("Attempting to run 3DMigoto Loader at: {:?}", migoto_exe),
    )
    .unwrap();
    let appc = app.clone();
    thread::spawn(move || {
        let output = Command::new(&migoto_exe).current_dir(&migoto_path).output();
        match output {
            Ok(out) => {
                if !out.status.success() {
                    appc.emit(
                        "log",
                        format!("3DMigoto Loader failed with exit code: {}", out.status),
                    )
                    .unwrap();
                } else {
                    appc.emit("log", "3DMigoto Loader started successfully.".to_string())
                        .unwrap();
                }
            }
            Err(e) => {
                appc.emit("log", format!("Failed to execute 3DMigoto Loader: {}", e))
                    .unwrap();
            }
        }
    });

    if let Some(game_path) = game_path {
        app.emit("log", format!("Attempting to run game at: {:?}", game_path))
            .unwrap();
        thread::spawn(move || {
            let parent_dir = game_path.parent().unwrap_or(&game_path);
            let output = Command::new(&game_path).current_dir(parent_dir).output();
            match output {
                Ok(out) => {
                    if !out.status.success() {
                        app.emit("log", format!("Game failed with exit code: {}", out.status))
                            .unwrap();
                    } else {
                        app.emit("log", "Game started successfully.".to_string())
                            .unwrap();
                    }
                }
                Err(e) => {
                    app.emit("log", format!("Failed to execute game: {}", e))
                        .unwrap();
                }
            }
        });
    }
    true
}

#[tauri::command]
fn add_mod(
    app: AppHandle,
    file_path: PathBuf,
    migoto_path: PathBuf,
    class: String,
    label: String,
    mod_name: String,
) {
    // 父目录
    let par_dir = migoto_path.join("Mods").join(&class).join(&label);

    if !par_dir.is_dir() {
        app.emit("log", par_dir.to_string_lossy().to_string())
            .unwrap();
        match create_dir_all(&par_dir) {
            Ok(_) => {
                app.emit(
                    "log",
                    format!("Create Dir:{}", &par_dir.to_string_lossy().to_string()),
                )
                .unwrap();
            }
            Err(e) => {
                app.emit("log", e.to_string()).unwrap();
            }
        }
    }

    let output = migoto_path
        .join("Mods")
        .join(&class)
        .join(&label)
        .join(&mod_name);

    app.emit("log", format!("Attempting to add mod to: {:?}", output))
        .unwrap();

    if let Err(e) = create_dir(&output) {
        if e.kind() != ErrorKind::AlreadyExists {
            app.emit("log", format!("Failed to create mod directory: {}", e))
                .unwrap();
            return;
        }
    }

    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        match ext {
            "7z" => {
                if let Err(e) = decompress_file(&file_path, &output) {
                    app.emit("log", format!("Failed to decompress 7z file: {}", e))
                        .unwrap();
                } else {
                    app.emit("log", "Successfully decompressed 7z mod file.".to_string())
                        .unwrap();
                }
            }
            "zip" => {
                if let Err(e) = unzip_file(&file_path, &output) {
                    app.emit("log", format!("Failed to unzip file: {}", e))
                        .unwrap();
                } else {
                    app.emit("log", "Successfully unzipped mod file.".to_string())
                        .unwrap();
                }
            }
            _ => {
                if file_path.is_dir() {
                    if let Err(e) = copy_dir_recursive(&file_path, &output) {
                        app.emit("log", format!("Failed to copy mod directory: {}", e))
                            .unwrap();
                    } else {
                        app.emit("log", "Successfully copied mod directory.".to_string())
                            .unwrap();
                    }
                } else {
                    app.emit("log", "Unsupported file type for mod.".to_string())
                        .unwrap();
                }
            }
        }
    } else if file_path.is_dir() {
        if let Err(e) = copy_dir_recursive(&file_path, &output) {
            app.emit("log", format!("Failed to copy mod directory: {}", e))
                .unwrap();
        } else {
            app.emit("log", "Successfully copied mod directory.".to_string())
                .unwrap();
        }
    } else {
        app.emit(
            "log",
            "Unsupported file or directory type for mod.".to_string(),
        )
        .unwrap();
    }
}

#[tauri::command]
fn get_mods(app: AppHandle, migoto_path: PathBuf, class: String, label: String) -> Vec<ModItem> {
    let mut rvc = Vec::new();
    let mods_path = migoto_path.join("Mods").join(&class).join(&label);
    app.emit(
        "log",
        format!("Attempting to get mods from: {:?}", mods_path),
    )
    .unwrap();

    if !mods_path.is_dir() {
        if let Err(e) = create_dir_all(&mods_path) {
            app.emit("log", format!("Failed to create mods directory: {}", e))
                .unwrap();
            return rvc;
        }
    }

    if let Ok(rdir) = read_dir(&mods_path) {
        for entry in rdir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.starts_with("DISABLED_") {
                        rvc.push(ModItem {
                            name: file_name.replace("DISABLED_", ""),
                            used: false,
                        });
                    } else {
                        rvc.push(ModItem {
                            name: file_name,
                            used: true,
                        });
                    }
                }
            } else if let Err(e) = entry {
                app.emit("log", format!("Error reading mods directory entry: {}", e))
                    .unwrap();
            }
        }
    } else if let Err(e) = read_dir(&mods_path) {
        app.emit("log", format!("Failed to read mods directory: {}", e))
            .unwrap();
    }
    rvc
}

#[tauri::command]
fn disable_mod(
    app: AppHandle,
    migoto_path: PathBuf,
    mod_name: String,
    class: String,
    label: String,
) {
    let from_path = migoto_path
        .join("Mods")
        .join(&class)
        .join(&label)
        .join(&mod_name);
    let to_path = migoto_path
        .join("Mods")
        .join(&class)
        .join(&label)
        .join(format!("DISABLED_{}", &mod_name));
    app.emit("log", format!("Attempting to disable mod: {:?}", from_path))
        .unwrap();
    if let Err(e) = rename(&from_path, &to_path) {
        app.emit(
            "log",
            format!("Failed to disable mod {:?}: {}", from_path, e),
        )
        .unwrap();
    } else {
        app.emit("log", "Mod disabled successfully.".to_string())
            .unwrap();
    }
}

#[tauri::command]
fn enable_mod(
    app: AppHandle,
    migoto_path: PathBuf,
    mod_name: String,
    class: String,
    label: String,
) {
    let from_path = migoto_path
        .join("Mods")
        .join(&class)
        .join(&label)
        .join(format!("DISABLED_{}", mod_name));
    let to_path = migoto_path
        .join("Mods")
        .join(&class)
        .join(&label)
        .join(&mod_name);
    app.emit("log", format!("Attempting to enable mod: {:?}", from_path))
        .unwrap();
    if let Err(e) = rename(&from_path, &to_path) {
        app.emit(
            "log",
            format!("Failed to enable mod {:?}: {}", from_path, e),
        )
        .unwrap();
    } else {
        app.emit("log", "Mod enabled successfully.".to_string())
            .unwrap();
    }
}

#[tauri::command]
fn delete_mod(
    app: AppHandle,
    migoto_path: PathBuf,
    mod_name: String,
    class: String,
    label: String,
) {
    let mod_path = migoto_path
        .join("Mods")
        .join(&class)
        .join(&label)
        .join(&mod_name);
    app.emit("log", format!("Attempting to delete mod: {:?}", mod_path))
        .unwrap();
    if let Err(e) = remove_dir_all(&mod_path) {
        app.emit(
            "log",
            format!("Failed to delete mod directory {:?}: {}", mod_path, e),
        )
        .unwrap();
    } else {
        app.emit("log", "Mod deleted successfully.".to_string())
            .unwrap();
    }
}

#[tauri::command]
fn get_previews(
    app: AppHandle,
    migoto_path: PathBuf,
    class: String,
    mod_name: String,
    label: String,
) -> Vec<String> {
    let mod_folder = migoto_path
        .join("Mods")
        .join(&class)
        .join(&label)
        .join(&mod_name);
    let mut rlist = Vec::new();
    if !mod_folder.is_dir() {
        app.emit("log", format!("Mod folder not found: {:?}", mod_folder))
            .unwrap();
        return rlist;
    }

    let preview_folder_path = format!("{}/preview", FOLDER);
    let preview_folder = Path::new(&preview_folder_path);
    app.emit(
        "log",
        format!("Attempting to get previews from: {:?}", mod_folder),
    )
    .unwrap();

    if preview_folder.is_dir() {
        if let Ok(count) = read_dir(preview_folder).map(|dir| dir.count()) {
            if count > 0 {
                if let Err(e) = remove_dir_all(preview_folder) {
                    app.emit("log", format!("Failed to remove old preview folder: {}", e))
                        .unwrap();
                    return rlist;
                }
            }
        } else if let Err(e) = read_dir(preview_folder) {
            app.emit("log", format!("Failed to read preview folder: {}", e))
                .unwrap();
            return rlist;
        }
    }

    if !preview_folder.is_dir() {
        if let Err(e) = create_dir_all(&preview_folder_path) {
            app.emit("log", format!("Failed to create new preview folder: {}", e))
                .unwrap();
            return rlist;
        }
    }

    if let Ok(entries) = read_dir(&mod_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                        if file_name.starts_with("preview") {
                            let dest_path = preview_folder.join(file_name);
                            if let Err(e) = copy(&path, &dest_path) {
                                app.emit(
                                    "log",
                                    format!(
                                        "Failed to copy preview file {:?} to {:?}: {}",
                                        path, dest_path, e
                                    ),
                                )
                                .unwrap();
                            } else {
                                rlist.push(file_name.to_string());
                            }
                        }
                    }
                }
            } else if let Err(e) = entry {
                app.emit("log", format!("Error reading mod directory entry: {}", e))
                    .unwrap();
            }
        }
    } else if let Err(e) = read_dir(&mod_folder) {
        app.emit(
            "log",
            format!("Failed to read mod folder for previews: {}", e),
        )
        .unwrap();
    }
    rlist
}

#[derive(Clone, Serialize, Deserialize)]
struct Character {
    name: String,
    weapon: Option<String>,
    area: Option<String>,
    star: Option<u8>,
    element: String,
    icon: Option<String>,
    label: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Wapons {
    name: String,
    label: String,
    kind: String,
    star: Option<u8>,
    icon: Option<String>,
}

/// 解压一个 zip 文件到指定的目录
fn unzip_file(zip_path: &Path, output_dir: &Path) -> io::Result<()> {
    // 确保输出目录存在
    std::fs::create_dir_all(output_dir)?;

    // 打开 zip 文件
    let zip_file = File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;

    // 遍历压缩包中的所有文件
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = output_dir.join(file.mangled_name());

        // 如果是目录，创建目录
        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            // 如果是文件，创建父目录并解压文件
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    // 确保目标目录存在，如果不存在则创建
    fs::create_dir_all(dst)?;

    // 遍历源目录中的所有文件和子目录
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();

        // 获取新路径
        let file_name = path
            .file_name()
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "Invalid file name"))?;
        let dst_path = dst.join(file_name);

        if path.is_dir() {
            // 如果是目录，递归调用自身
            copy_dir_recursive(&path, &dst_path)?;
        } else {
            // 如果是文件，直接复制文件
            fs::copy(&path, &dst_path)?;
        }
    }
    Ok(())
}

type Name = Vec<Vec<String>>;

#[derive(Serialize, Deserialize)]
struct Game {
    name: Name,
    icon: String,
    bg: String,
    value: String,
    game_path: PathBuf,
    migoto_path: PathBuf,
    classes: Vec<Class>,
}

#[derive(Serialize, Deserialize)]
struct Filter {
    name: Name,
    key: String,
    list: Vec<FilterItem>,
}
#[derive(Serialize, Deserialize)]
struct FilterItem {
    name: Name,
    value: String,
    icon: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Class {
    name: Name,
    value: String,
    icon: String,
    filters: Option<Vec<Filter>>,
    list: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
struct ModItem {
    name: String,
    used: bool,
}
