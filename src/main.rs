use actix_web::{App, HttpServer, web::Data, dev::ServiceRequest, Error};

mod routes;

use routes::{root};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move ||{
        App::new()
            .service(root)
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
