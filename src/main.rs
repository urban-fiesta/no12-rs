mod app;
mod handlers;
mod models;
mod routes;

/// Provides a RESTful web server for managing Events.
///
/// API will be:
///
/// - `GET /events`: return a JSON list of Events.
/// - `POST /events`: create a new Event.
/// - `PUT /events/:id`: update a specific Event.
/// - `DELETE /events/:id`: delete a specific Event.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::init();

    app::start().await;
}
