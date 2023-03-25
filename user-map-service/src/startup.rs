use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;

use std::net::TcpListener;
use crate::routes::{health_check, history};
use aws_sdk_dynamodb::Client;

// create one connection
// to a client
// then share

//impl FromRequest for Client {
    // associated types
    // type parameters bound to struct you are
    // implementing
    // every client uses associated types
 //   type Error = actix_web::Error;
    // traits and associated types
    //can override if want
    //type Error: Into<Error>;
    //type Future: Future<Output = Result<Self, Self::Error>>;
    
    // extending existing types
    // impl client
  //  fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
//        Ok(client)
   // }
//}

pub fn run(listener: TcpListener, client: Client) -> Result<Server, std::io::Error> {
    let db_client = Data::new(client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .service(
                web::resource("/history")
                    .app_data(db_client.clone())
                    .route(web::get().to(history)
                    )
            )
    })
    .listen(listener)?
    .run();
    // this returns an Ok that wraps an impl Future
    Ok(server)
}
