use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemModel {
    pub name: Option<String>,
    pub path: String,
}
impl ItemModel {
    pub fn as_string(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else {
            self.path.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub bookmarks: Vec<ItemModel>,
}

pub struct Db {
    path: PathBuf,
    pub data: Model,
}
impl Db {
    pub fn new() -> Self {
        let home = home_dir().unwrap();
        let path = home.join(".dirbkm");
        Self {
            path,
            data: Model {
                bookmarks: Vec::new(),
            },
        }
    }
    pub fn read_file(&mut self) {
        if let Ok(mut file) = std::fs::File::open(&self.path) {
            let mut data = String::new();

            if file.read_to_string(&mut data).is_ok() {
                let toml: Model = toml::from_str(&data).unwrap();
                self.data = toml;
            }
        }
    }
    pub fn write_file(&mut self) {
        let toml = toml::to_string(&self.data).unwrap();

        let mut file = std::fs::File::create(&self.path).unwrap();

        file.write_all(toml.as_bytes()).unwrap();
    }
    pub fn add(&mut self, name: Option<String>, path: String) {
        self.data.bookmarks.push(ItemModel { name, path });
    }
}

fn home_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").and_then(|h| if h.is_empty() { None } else { Some(h) });
    if let Some(home) = home {
        Some(PathBuf::from(home))
    } else {
        None
    }
}
