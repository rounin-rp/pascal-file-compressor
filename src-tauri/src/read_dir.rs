use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrFile {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

pub struct Directory {
    pub name: String,
    pub path: PathBuf,
}

impl Directory {
    pub fn new() -> Self {
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
            curr_files.push(CurrFile {
                name,
                path: path.to_str().unwrap().to_string(),
                is_dir,
            })
        }
        curr_files
    }
}

#[tauri::command]
pub fn read_dir() -> Vec<CurrFile> {
    let directory = Directory::new();
    directory.read_current_dir()
}
