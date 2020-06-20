use warp::{self, Filter};

use crate::handlers;
use crate::models::{Db, ListOptions};

pub fn routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(handlers::index)
        .or(warp::path!("api" / "events")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_db(db.clone()))
            .and_then(handlers::list_events))
        .or(warp::path!("api" / "events")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::create_event))
        .or(warp::path!("api" / "events" / u64)
            .and(warp::put())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::update_event))
        .or(warp::path!("api" / "events" / u64)
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handlers::delete_event))
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
