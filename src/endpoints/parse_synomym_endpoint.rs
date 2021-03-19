use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Request {
    pub synonym: String
}

#[derive(Serialize)]
pub struct Response {
    status: bool,
    category: String
}

// This endpoint checks if given category is a
// synonym for existing category
pub async fn response(info: web::Query<Request>) -> impl Responder {

    let map = get_synonym_map();

    if map.contains_key(&info.synonym) {
        web::HttpResponse::Ok()
            .json(Response { status: true, category: map.get(&info.synonym).unwrap().clone() })
    } else {
        web::HttpResponse::Ok()
            .json(Response { status: false, category: "".to_string() })
    }
}

// returns a HashMap of all synonyms
fn get_synonym_map() -> HashMap<String, String> {
    let raw = std::fs::read_to_string("./synonyms.txt").unwrap();
    let spl = raw.split("\n").collect::<Vec<&str>>();
    let mut map: HashMap<String, String> = HashMap::new();
    for el in spl {
        let sep = el.split("=>").collect::<Vec<&str>>();
        map.insert(sep[0].to_string(), sep[1].to_string());
    }
    return map;
}