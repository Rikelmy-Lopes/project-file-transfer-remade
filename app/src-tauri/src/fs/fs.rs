use serde::{Deserialize, Serialize};
use std::{
    env::home_dir,
    fs::{self, FileType},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub struct File {
    name: String,
    path: String,
    filetype: String,
}

fn get_file_type(file: FileType) -> String {
    if file.is_file() {
        return String::from("file");
    } else {
        return String::from("directory");
    }
}

#[tauri::command]
fn is_dir(dir: String) -> bool {
    let path = PathBuf::from(dir);
    let is_dir = path.is_dir();
    return is_dir;
}

pub fn get_dir_entries() -> Vec<File> {
    let paths = fs::read_dir(home_dir().unwrap()).unwrap();
    let mut files: Vec<File> = Vec::new();

    for path in paths {
        let dir_entry = path.unwrap();
        files.push(File {
            name: dir_entry.file_name().to_str().unwrap().to_owned(),
            path: dir_entry.path().to_str().unwrap().to_owned(),
            filetype: get_file_type(dir_entry.file_type().unwrap()),
        });
    }

    return files;
}
