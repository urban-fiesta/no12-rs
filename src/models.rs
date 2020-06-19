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
    pub description: String,
    pub age_limit: u32,
    pub event_photo: String,
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
}

/// A simple in-memory DB, a vector synchronized by a mutex.
pub type Db = Arc<Mutex<Vec<Event>>>;

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

// impl User {
//     pub fn new(id: u64, name: String, email: String, password: String) -> Self {
//         User {
//             id,
//             name,
//             email,
//             password,
//         }
//     }
// }

// impl Event {
//     pub fn new(id: u64, name: String, date: String, local: String, start: String, finish: String, price: f32, description: String, age_limit: u32, event_photo: String) -> Self {
//         Event {
//             id,
//             name,
//             date,
//             local,
//             start,
//             finish,
//             price,
//             description,
//             age_limit,
//             event_photo,
//         }
//     }
// }
