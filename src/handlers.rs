use std::convert::Infallible;
use warp::http::StatusCode;

use crate::models::{Db, Event, ListOptions};

pub fn index() -> &'static str {
    "Hello world!"
}

pub async fn list_events(opts: ListOptions, db: Db) -> Result<impl warp::Reply, Infallible> {
    let events = db.lock().await;
    let events: Vec<Event> = events
        .clone()
        .into_iter()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(std::usize::MAX))
        .filter(|event| match &opts.tag {
            Some(tag) => event.tag_list.contains(&tag),
            None => true,
        })
        .collect();
    Ok(warp::reply::json(&events))
}

pub async fn get_event_by_id(id: u64, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let events = db.lock().await;

    for event in events.iter() {
        if event.id == id {
            return Ok(Box::new(warp::reply::json(&event)));
        }
    }
    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn create_event(new: Event, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut events = db.lock().await;

    for event in events.iter() {
        if event.id == new.id {
            // Event with id already exists, return `400 BadRequest`.
            return Ok(StatusCode::BAD_REQUEST);
        }
    }
    events.push(new);

    Ok(StatusCode::CREATED)
}

pub async fn update_event(id: u64, update: Event, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("update_event: id={}, event={:?}", id, update);
    let mut events = db.lock().await;

    // Look for the specified Event...
    for event in events.iter_mut() {
        if event.id == id {
            *event = update;
            return Ok(StatusCode::OK);
        }
    }

    // If the for loop didn't return OK, then the ID doesn't exist...
    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_event(id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut events = db.lock().await;

    let len = events.len();

    // Remove all events with this id
    events.retain(|event| event.id != id);

    let deleted = events.len() != len;

    if deleted {
        // respond with a `204 No Content`, which means successful,
        // yet no body expected...
        Ok(StatusCode::NO_CONTENT)
    } else {
        log::debug!("    -> event id not found!");
        Ok(StatusCode::NOT_FOUND)
    }
}
