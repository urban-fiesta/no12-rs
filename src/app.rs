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
}
*/

pub async fn start() {
    /*
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let repository = Repository(Repo::new(&database_url));

    let _app_state = AppState {
        repository,
    };
    */

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_credentials(true)
        .allow_header("content-type")
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    let db = models::blank_db();

    let routes = routes::routes(db)
        .with(warp::log(APPLICATION_NAME))
        .with(cors);

    warp::serve(routes).run(bind_address).await;
}
