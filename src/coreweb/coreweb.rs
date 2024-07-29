mod handler;
pub mod parse;
pub mod types;
use actix_web::{App, HttpServer};
use handler::app_config;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server run at http:://0.0.0.0:3000");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            // .allowed_origin("http://localhost:3000")
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new().wrap(cors).configure(app_config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
