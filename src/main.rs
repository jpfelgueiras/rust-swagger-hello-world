use std::{error::Error, net::SocketAddr};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use axum::{
    routing::get,
    Router,
};

#[derive(OpenApi)]
#[openapi(paths(hello_axum))]
pub struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let app = Router::new()
        .route("/", get(hello_axum))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Send a salute from Axum")
    )
)]
pub async fn hello_axum() -> impl IntoResponse {
    (StatusCode::OK, "Hello, Axum")
}
