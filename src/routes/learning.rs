use axum::{ routing::post, routing:: get, Router };

pub fn learning() -> Router {
  Router::new().route("/learn", get("get learn"))
  .route("/learn", post("post learn"))
}