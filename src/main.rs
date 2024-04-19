use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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

struct AppState {
    db: Mutex<DataBase>,
}

async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(task.into_inner()); // into_inner là kiểu unwrap, json parse á
    let _ = db.save_to_file();

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match DataBase::load_from_file() {
        Ok(db) => db,
        Err(_) => DataBase::new(),
    };

    let data: web::Data<AppState> = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/task", web::post().to(create_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
