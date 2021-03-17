use actix_web::{HttpServer, App, web, HttpRequest, Result};
use actix_cors::Cors;
use dotenv::dotenv;

mod endpoints;
mod env_handler;
mod structs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .route("/api", web::get().to(endpoints::default_endpoint::response))
            .route("/api/fetch", web::get().to(endpoints::fetch_endpoint::response))
            .route("/api/parse_synonym", web::get().to(endpoints::parse_synomym_endpoint::response))
    })
        .bind("0.0.0.0:".to_owned() + &env_handler::load_param("APPLICATION_PORT"))?
        .run()
        .await
}
