use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::structs::category_content::CategoryContent;
use crate::structs::letter_entry::LetterEntry;
use serde_json::to_string;

#[derive(Deserialize)]
pub struct Request {
    pub category: String,
    pub letter: String
}

#[derive(Serialize)]
struct Response {
    status: bool,
    message: String,
    data: Vec<LetterEntry>
}


pub async fn response(info: web::Query<Request>) -> impl Responder {
    let path = format!("./categories/{}.json", &info.category);
    if path_exists(&path) {
        let file_content = std::fs::read_to_string(&path).unwrap();
        let json: serde_json::Result<CategoryContent>  = serde_json::from_str(&*file_content);
        web::HttpResponse::Ok()
            .json(Response {
                status: true,
                message: "Successfully queried data".to_string(),
                data: get_vec_by_char(json.unwrap(), &*info.letter)
            })
    } else {
        web::HttpResponse::Ok()
            .json(Response {
                status: false,
                message: "Category not found".to_string(),
                data: vec![]
            })
    }
}

fn path_exists(path: &String) -> bool {
    std::path::Path::new(path).exists()
}

fn get_vec_by_char(model: CategoryContent, category: &str) -> Vec<LetterEntry> {
    return match category {
        "a" => { model.a }
        _ => { vec![] }
    };
}