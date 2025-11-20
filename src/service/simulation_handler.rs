use axum::{Json, response::IntoResponse};
use crate::models::SimConfig;
use crate::models::SimStop;

pub async fn sim_start_handler(Json(cfg): Json<SimConfig>) -> impl IntoResponse {
  println!("============== 시뮬레이션 시작 ==============");
  println!("차선 수: {}, 장매물: {}개 {}초, 반복 횟수: {}", cfg.lane, cfg.obs, cfg.frequency, cfg.learn);
  println!("=============================================");
}

pub async fn sim_stop_handler(Json(cfg): Json<SimStop>) -> impl IntoResponse {
  println!("============== 시뮬레이션 중지 ==============");
  println!("시뮬레이션 STATE: {}", cfg.sim_state);
  println!("=============================================");
}