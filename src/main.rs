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

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;
    use warp::{self, Filter};

    use crate::models::{self, Event};
    use crate::routes;

    fn event1() -> Event {
        Event {
            id: 1,
            name: "test event".into(),
            date: "16/06/20".into(),
            local: "test local".into(),
            start: "18h".into(),
            finish: "20h".into(),
            price: 50.0,
            description: None,
            age_limit: None,
            event_photo: None,
        }
    }

    fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let db = models::blank_db();
        routes::routes(db)
    }

    #[tokio::test]
    async fn create_event() {
        let api = api();

        let resp = request()
            .method("POST")
            .path("/api/events")
            .json(&event1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn get_all_events() {
        let api = api();

        let resp = request()
            .method("GET")
            .path("/api/events")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_event_by_id() {
        let api = api();
        let id: i32 = 1;

        request()
            .method("POST")
            .path("/api/events")
            .json(&event1())
            .reply(&api)
            .await;

        let resp = request()
            .method("GET")
            .path(format!("/api/events/{}", id).as_str())
            .json(&event1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
