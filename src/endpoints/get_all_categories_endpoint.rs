use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    categories: Vec<String>
}

pub async fn response() -> impl Responder {

    let paths = std::fs::read_dir("./categories").unwrap();
    let mut resp: Vec<String> = vec![];
    for path in paths {
        resp.push(format!("{}",
                          path.unwrap().path().display()).split("\\").collect::<Vec<&str>>()[1]
            .split(".").collect::<Vec<&str>>()[0].to_string()
        );
    }
    web::HttpResponse::Ok()
        .json(Response {
            categories: resp,
        })
}