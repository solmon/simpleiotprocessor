use super::*;

pub async fn launch() -> impl Responder {
    let data = serde_json::json!({
        "name" :"solly"
    });
    web::Json(data)
}
