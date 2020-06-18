mod models;

use warp::Filter;

use self::models::{Event, User};

#[tokio::main]
async fn main() {
    let db = models::blank_db();
    let api = filters::events(db);
    let routes = api.with(warp::log("events"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {
    use super::handlers;
    use crate::models::{Db, Event, ListOptions};
    use warp::Filter;

    pub fn events(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        events_list(db.clone()).or(event_create(db.clone()))
    }

    pub fn events_list(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("events")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_db(db))
            .and_then(handlers::list_events)
    }

    pub fn event_create(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("events")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db))
            .and_then(handlers::create_event)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use crate::models::{Db, Event, ListOptions};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn list_events(opts: ListOptions, db: Db) -> Result<impl warp::Reply, Infallible> {
        let events = db.lock().await;
        let events: Vec<Event> = events
            .clone()
            .into_iter()
            .skip(opts.offset.unwrap_or(0))
            .take(opts.limit.unwrap_or(std::usize::MAX))
            .collect();
        Ok(warp::reply::json(&events))
    }

    pub async fn create_event(new: Event, mut db: Db) -> Result<impl warp::Reply, Infallible> {
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

    pub async fn update_event(
        id: u64,
        update: Event,
        db: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        //log::debug!("update_todo: id={}, todo={:?}", id, update);
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
        events.retain(|event| {
            // Retain all Events that aren't this id...
            // In other words, remove all that *are* this id...
            event.id != id
        });

        // If the vec is smaller, we found and deleted a Event!
        let deleted = events.len() != len;

        if deleted {
            // respond with a `204 No Content`, which means successful,
            // yet no body expected...
            Ok(StatusCode::NO_CONTENT)
        } else {
            //log::debug!("    -> event id not found!");
            Ok(StatusCode::NOT_FOUND)
        }
    }
}
