use actix_web::http::header::CONTENT_TYPE;
use actix_web::HttpServer;
use actix_web::{Responder,HttpResponse};
use actix_web::{delete, get, post, web};



#[post("/internal/exit")]
#[allow(unreachable_code)]
async fn exit() -> impl Responder {
    log::info!("Shutdown as requested");
    std::process::exit(0);
    format!("Exited")
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(hello);

}



#[get("/hello")]
async fn hello(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("hello boy")
}