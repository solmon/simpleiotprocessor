use actix_web::Responder;
use actix_web::{web, web::ServiceConfig};

mod index;
use index::*;

pub fn app_config(config: &mut ServiceConfig) {
    let launch = web::resource("/device").route(web::get().to(launch));
    config.service(launch);
    config.service(actix_files::Files::new("/", "./src/coreweb/static").index_file("index.html"));
}
