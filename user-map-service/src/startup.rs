use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use std::net::TcpListener;

use crate::routes::{health_check, history};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
//            .route("/history", web::post().to(history))
            .service(history)
    })
    .listen(listener)?
  .run();
    Ok(server)
}
