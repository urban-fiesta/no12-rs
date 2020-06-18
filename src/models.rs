extern crate serde;

use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub start: String,
    pub finish: String,
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

/// A simple in-memory DB, a vector synchronized by a mutex.
pub type Db = Arc<Mutex<Vec<Event>>>;

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        User {
            name,
            email,
            password,
        }
    }
}

impl Event {
    pub fn new(name: String, start: String, finish: String) -> Self {
        Event {
            name,
            start,
            finish,
        }
    }
}
