use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde_derive::{Deserialize, Serialize};

use crate::constants::DB_FILE_NAME;

pub struct Database {
    pub passwords: Vec<Password>,
}

impl Database {
    pub fn new() -> Self {
        let path = Path::new(DB_FILE_NAME);
        let file = File::open(path);

        let mut passwords = Vec::new();

        match file {
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => panic!("{}", e.to_string()),
            },
            Ok(f) => passwords = Database::load_passwords(f),
        }

        Self { passwords }
    }

    pub fn save_to_file(&self) {
        let path = Path::new(DB_FILE_NAME);
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();

        let json = serde_json::to_string(&self.passwords).unwrap();

        file.write(json.as_bytes())
            .expect("Could not write to file!");
    }

    pub fn create_password(&mut self, new_password: Password) {
        self.passwords.push(new_password);

        self.save_to_file();
    }

    pub fn read_passwords(&self) {
        for password in self.passwords.iter() {
            println!("{:?}", password);
        }
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

    pub fn delete_password(&mut self, name: String) {
        for i in 0..self.passwords.len() {
            if self.passwords[i].name == name {
                self.passwords.remove(i);
                break;
            }
        }

        self.save_to_file();
    }

    fn load_passwords(mut f: File) -> Vec<Password> {
        //TODO might need to update if performance will be a problem in the future
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();

        let passwords: Vec<Password> =
            serde_json::from_str(&content).expect("JSON was not well-formatted");

        passwords
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Password {
    pub name: String,
    pub content: String,
}

impl Password {
    pub fn new(name: String, content: String) -> Self {
        Password { name, content }
    }
}
