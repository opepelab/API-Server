use actix_web::{get, App, HttpResponse, HttpServer};
use serde::Serialize;
use chrono::Local;



#[derive(Serialize)]
struct MyObj {
    slug: String,
    title: String,
    body: String,
    date: String,
    // num: isize, 整数型 usizeもある
    // arr: Vec::<isize>,　変更可能な配列型　ただの固定配列型もある？
}
 
#[get("/getjson")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(MyObj {
        slug: "fetch-pr".to_string(), //dynamicrouting[slug].tsx
        title: "たいとるなでしこ".to_string(),
        body: "やんくっく".to_string(),
        date:  Local::now().format("%Y年%m月%d日 %H時%M分%S秒").to_string(),
        // num: 100,
        // arr: vec![1, 2, 3],
    })
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        // localhostからのみアクセスするなら127.0.0.1
        // .bind("127.0.0.1:8080")?
        // EC2等に配置して外部からアクセスするなら0.0.0.0
        .bind("0.0.0.0:8080")?
        .run()
        .await
}