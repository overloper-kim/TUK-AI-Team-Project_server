use axum:: { routing::get, Router, };

pub fn simulation() -> Router {
    Router::new().route("/configure", get("check")).
    route("/sim_start", get("simulation start")).
    route("/sim_stop", get("simulation stop")).
    route("/sim_pause", get("simulation pause")).
    route("/get_train", get("get training data"))
}