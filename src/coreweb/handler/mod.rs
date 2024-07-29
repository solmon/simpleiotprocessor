use actix_web::Responder;
use actix_web::{web, web::ServiceConfig};

mod index;
mod launch;

use index::*;
use launch::*;

use crate::parse::ResponseData;

pub fn app_config(config: &mut ServiceConfig) {
    let launch = web::resource("/device").route(web::get().to(launch));
    let device = web::resource("/launch").route(web::get().to(index));
    config.service(launch);
    config.service(device);
    config.service(actix_files::Files::new("/", "./src/coreweb/static").index_file("index.html"));
}
