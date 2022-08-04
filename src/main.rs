use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
// use chrono::Local;



#[derive(Serialize)]
struct Country {
    id: isize,
    slug: String,
    title: String,
    body: String
}
 
async fn get_country_list()  -> impl Responder {
    let mut vec:Vec<Country> = Vec::new();
 
    vec.push(Country{id: 1, slug: "CTX".to_string(), title: "nekko".to_string(), body: "Philippines".to_string()});
    vec.push(Country{id: 2, slug: "CTX".to_string(), title: "nekko".to_string(), body: "Philippines".to_string()});
 
    return web::Json(vec);
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/countries", web::get().to(get_country_list))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}