use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use listenfd::ListenFd;

mod employees;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));

    server.bind("127.0.0.1:8001")?
    .run()
    .await
}

