// use serde::{Serialize, Deserialize};
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Write;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{spawn, JoinHandle};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataBase {
    task: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl DataBase {
    fn new() -> Self {
        Self {
            task: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn insert(&mut self, task: Task) {
        self.task.insert(task.id, task);
    }

    fn get(&self, id: &u64) -> Option<&Task> {
        self.task.get(id)
    }

    fn get_all(&self) -> Vec<&Task> {
        self.task.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.task.remove(id);
    }

    // update is replaced
    fn update(&mut self, task: Task) {
        self.task.insert(task.id, task);
    }

    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }
    // DATABASE SAVING
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Self = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

fn main() {
    println!("Hello, world!");
}
