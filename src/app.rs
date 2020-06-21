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
    /*
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let repository = Repository(Repo::new(&database_url));
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let _app_state = AppState {
        repository,
        jwt_secret,
    };
    */

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["*"])
        .allow_methods(vec!["POST"]);

    let db = models::blank_db();

    let routes = routes::routes(db)
        .with(warp::log(APPLICATION_NAME))
        .with(cors);

    warp::serve(routes).run(bind_address).await;
}
