use axum:: {
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 라우터 생성
    let app = Router::new().route("/", get("hello world"));

    // 서버 바인딩 설정
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("서버가 {} 에서 실행되고 있습니다.", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(),app).await.unwrap();
}