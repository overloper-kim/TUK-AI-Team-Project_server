// serde -> 직렬화/역직렬화 라이브러리
// chrono -> 날짜/시간 라이브러리
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct SimConfig {
    pub lane: u8,
    pub obs: u8,
    pub frequency: u8,
    pub learn: u16,
}

#[derive(Debug, Deserialize)]
pub struct SimStop {
    pub sim_state: String,
}