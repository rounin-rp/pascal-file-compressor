use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
extern crate dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrFile {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_compressed: bool,
}
#[derive(Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub path: PathBuf,
}

impl Directory {
    pub fn new() -> Self {
        let curr_dir = dirs::home_dir().unwrap();
        Self {
            name: curr_dir.file_name().unwrap().to_str().unwrap().to_string(),
            path: curr_dir,
        }
    }
    pub fn from(current_dir: &CurrFile) -> Self {
        std::env::set_current_dir(current_dir.path.clone());
        let curr_dir = std::env::current_dir().unwrap();
        Self {
            name: curr_dir.file_name().unwrap().to_str().unwrap().to_string(),
            path: curr_dir,
        }
    }
    pub fn back(&mut self) {
        self.path.pop();
        self.name = self.path.file_name().unwrap().to_str().unwrap().to_string();
        std::env::set_current_dir(self.path.clone());
    }
    pub fn read_current_dir(&self) -> Vec<CurrFile> {
        let mut curr_files: Vec<CurrFile> = Vec::new();
        let current_path = fs::read_dir(self.path.clone()).unwrap();
        for entry in current_path {
            let path = entry.unwrap().path();
            let name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or("no_name")
                .to_string();
            let is_dir = path.is_dir();
            let is_compressed = name.ends_with(".pcompressed");
            if name.starts_with(".") {
                continue;
            }
            curr_files.push(CurrFile {
                name,
                path: path.to_str().unwrap().to_string(),
                is_dir,
                is_compressed,
            })
        }
        curr_files
    }
}

#[tauri::command]
pub fn read_dir(curr_dir: Option<Directory>) -> (Vec<CurrFile>, Directory) {
    match curr_dir {
        Some(dir) => (dir.read_current_dir(), dir),
        None => {
            let mut dir = Directory::new();
            (dir.read_current_dir(), dir)
        }
    }
}

#[tauri::command]
pub fn click_dir(selected_dir: CurrFile) -> (Vec<CurrFile>, Directory) {
    let directory = Directory::from(&selected_dir);
    (directory.read_current_dir(), directory)
}

#[tauri::command]
pub fn back_dir(curr_dir: Option<Directory>) -> (Vec<CurrFile>, Directory) {
    match curr_dir {
        Some(mut dir) => {
            dir.back();
            (dir.read_current_dir(), dir)
        }
        None => {
            let mut dir = Directory::new();
            dir.back();
            (dir.read_current_dir(), dir)
        }
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("read_dir")
        .invoke_handler(tauri::generate_handler![read_dir, click_dir, back_dir])
        .build()
}
