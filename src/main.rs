pub mod routes;

use axum:: {
    Router,
};
use std::{net::SocketAddr};
use routes::simulation::simulation;

use std::{env};

#[tokio::main]
async fn main() {
    // 라우터 생성
    let app: Router = simulation();

    // 서버 바인딩 설정
    let default_addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let addr = split_args(default_addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(),app).await.unwrap();
}

fn split_args(default_addr: SocketAddr) -> SocketAddr {
    let args: Vec<String> = env::args().collect();

    if args.len() > 6 {
        println!("잘못된 인자 값입니다.");
        println!("cargo run **.rs -ip addrss -p port");
        // exit program
        std::process::exit(1);
    }
    if args.len() == 1 {
        println!("서버가 {} 에서 실행되고 있는 중입니다.", default_addr);
        return default_addr;
    }
    if args.len() == 4 {
        let ip_arg: String = args.get(3).unwrap().to_string();
        let new_addr: SocketAddr = format!("{}:{}", ip_arg, default_addr.port()).parse().expect("IP format error");
        println!("서버가 {} 에서 실행되고 있는 중입니다.", new_addr);
        return new_addr;
    }
    if args.len() == 6 {
        let ip_arg: String = args.get(3).unwrap().to_string();
        let port: u16 = args.get(5).unwrap().parse().expect("port error");
        let new_addr: SocketAddr = format!("{}:{}", ip_arg, port).parse().expect("Port format error");
        println!("서버가 {} 에서 실행되고 있는 중입니다.", new_addr);
        return new_addr;
    }

    default_addr
}