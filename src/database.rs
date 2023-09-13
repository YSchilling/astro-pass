use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
    path::Path,
};

use serde_derive::{Deserialize, Serialize};

use crate::constants::DB_FILE_NAME;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub id_counter: u32,
    pub passwords: Vec<Password>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            id_counter: 0,
            passwords: Vec::new(),
        }
    }
    pub fn load_from_file() -> Self {
        // open file
        let path = Path::new(DB_FILE_NAME);
        let file_res = File::open(path);

        // check file error
        let mut file = match file_res {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => return Database::new(),
                _ => panic!("{}", e.to_string()),
            },
        };

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        serde_json::from_str(&content).expect("JSON was not well-formatted")
    }

    pub fn save_to_file(&self) {
        let path = Path::new(DB_FILE_NAME);
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();

        let json = serde_json::to_string(&self).unwrap();

        file.write(json.as_bytes())
            .expect("Could not write to file!");
    }

    pub fn create_password(&mut self, new_password: Password) {
        self.passwords.push(new_password);

        self.save_to_file();
    }

    pub fn update_password(&mut self, name: String, new_password: Password) {
        for i in 0..self.passwords.len() {
            if self.passwords[i].name == name {
                self.passwords[i] = new_password;
                break;
            }
        }

        self.save_to_file();
    }

    pub fn delete_password(&mut self, id: u32) {
        for i in 0..self.passwords.len() {
            if self.passwords[i].id == id {
                self.passwords.remove(i);
                break;
            }
        }

        self.save_to_file();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Password {
    pub id: u32,
    pub name: String,
    pub content: String,
}

impl Password {
    pub fn new(id: u32, name: String, content: String) -> Self {
        Password { id, name, content }
    }
}
