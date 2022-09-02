use actix_web::{App, HttpServer};

mod routes;

use routes::{root, status, hook};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move ||{
        App::new()
            .service(root)
            .service(status)
            .service(hook)
    })
        .bind("0.0.0.0:8080")
        .unwrap()
        .run()
        .await
}
