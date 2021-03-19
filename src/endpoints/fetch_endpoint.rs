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

// Endpoint for fetching category data
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

// function to check if file exists
fn path_exists(path: &String) -> bool {
    std::path::Path::new(path).exists()
}


// parses CategoryModel to Vec<LetterEntry> by letter
fn get_vec_by_char(model: CategoryContent, letter: &str) -> Vec<LetterEntry> {
    return match letter {
        "a" => { model.a },
        "b" => { model.b },
        "c" => { model.c },
        "d" => { model.d },
        "e" => { model.e },
        "f" => { model.f },
        "g" => { model.g },
        "h" => { model.h },
        "i" => { model.i },
        "j" => { model.j },
        "k" => { model.k },
        "l" => { model.l },
        "m" => { model.m },
        "n" => { model.n },
        "o" => { model.o },
        "p" => { model.p },
        "q" => { model.q },
        "r" => { model.r },
        "s" => { model.s },
        "t" => { model.t },
        "u" => { model.u },
        "v" => { model.v },
        "w" => { model.w },
        "x" => { model.x },
        "y" => { model.y },
        "z" => { model.z },
        _ => { vec![] }
    };
}