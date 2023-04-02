// /api
use super::{search::search, utils::Message};
use poem::{get, handler, http::StatusCode, web::Json, IntoResponse, Route};

#[handler]
fn index() -> impl IntoResponse {
    let mut message = Message::default();
    message
        .with_message("Search package information with files those include in package")
        .with_status(StatusCode::OK)
        .insert_usage("{protocol}://{host}:{port(default: 80/443)}/api/search?f=pacman")
        .with_demo();
    Json(message).with_status(StatusCode::OK).into_response()
}

pub fn api() -> Route {
    Route::new().at("/search", get(search)).at("/", get(index))
}
