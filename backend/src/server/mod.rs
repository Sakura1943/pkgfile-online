pub mod api;
pub mod search;
pub mod utils;
use poem::{
    error::NotFoundError,
    get, handler,
    http::{Method, StatusCode},
    listener::TcpListener,
    middleware::{Cors, Tracing},
    web::Json,
    EndpointExt, IntoResponse, Route, Server,
};

use self::utils::RequestFailed;

// Running server
pub async fn run(port: u16, host: &str) -> std::result::Result<(), std::io::Error> {
    let cors = Cors::default().allow_methods([Method::GET]);

    let app = Route::new()
        .nest("/api", api::api())
        .at("/", get(index))
        .at("/ping", get(ping))
        .with(cors)
        .with(Tracing)
        .catch_error(|_: NotFoundError| async move {
            Json(RequestFailed::default().with_error_msg("Page Not Found!"))
                .with_status(StatusCode::NOT_FOUND)
                .into_response()
        });

    Server::new(TcpListener::bind(format!("{host}:{port}")))
        .run(app)
        .await
}

// Home page
#[handler]
fn index() -> impl IntoResponse {
    let mut message = utils::Message::default();
    message
        .with_message("An api to request arch linux package information with file name")
        .with_status(StatusCode::OK)
        .insert_usage("{protocol}://{host}:{port(default: 80/443)}/api/search?f=pacman")
        .with_demo();
    Json(message).with_status(StatusCode::OK).into_response()
}

// Confirm server is active and runnning
#[handler]
fn ping() -> impl IntoResponse {
    let mut message = utils::Message::default();
    message.with_status(StatusCode::OK).with_message("pong!");
    Json(message).with_status(StatusCode::OK).into_response()
}
