extern crate serde;

use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: u64,
    pub name: String,
    pub date: String,
    pub local: String,
    pub start: String,
    pub finish: String,
    pub price: f32,
    pub tag_list: Vec<String>,
    pub description: Option<String>,
    pub age_limit: Option<u32>,
    pub event_photo: Option<String>,
}

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub tag: Option<String>,
}

/// A simple in-memory DB, a vector synchronized by a mutex.
pub type Db = Arc<Mutex<Vec<Event>>>;

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}
