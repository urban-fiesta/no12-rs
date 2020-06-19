use std::env;
use std::net::SocketAddr;

use warp::{self, Filter};

use crate::models;
use crate::routes;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

/*
#[derive(Clone)]
pub struct AppState {
    pub repository: Repository,
    pub jwt_secret: String,
}
*/

pub async fn start() {
    /* Very soon we'll be using this, so I'm leaving this here
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let repository = Repository(Repo::new(&database_url));
    let app_state = AppState {
        repository,
        jwt_secret,
    };
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    */

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let db = models::blank_db();
    let routes = routes::routes(db).with(warp::log(APPLICATION_NAME));

    warp::serve(routes).run(bind_address).await;
}
