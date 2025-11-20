use axum:: { routing::get, routing::post, Router };
use crate::service::simulation_handler::{self, sim_stop_handler};

pub fn simulation() -> Router {
    Router::new().route("/configure", get("check")).
    route("/sim_start", post(simulation_handler::sim_start_handler)).
    route("/sim_stop", post(simulation_handler::sim_stop_handler)).
    route("/sim_pause", get("simulation pause")).
    route("/get_train", get("get training data"))
}