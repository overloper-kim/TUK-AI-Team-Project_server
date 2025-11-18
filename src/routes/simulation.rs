use axum:: { routing::get, Router, };

pub fn get_sim_config() -> Router {
    Router::new()
    .route("/configure", get("check"))
}