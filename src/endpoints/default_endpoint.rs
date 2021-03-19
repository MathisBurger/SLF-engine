use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
    version: String,
}

// default response endpoint
pub async fn response() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Response {
            message: "Backend running...".to_string(),
            version: "v1.0.0".to_string()
        })
}