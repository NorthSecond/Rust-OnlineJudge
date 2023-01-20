use actix_web::http::header::CONTENT_TYPE;
use actix_web::HttpServer;
use actix_web::{delete, get, post, web};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

mod userHandler;
mod problemHandler;
mod submissionHandler;

#[post("/internal/exit")]
#[allow(unreachable_code)]
async fn exit() -> impl Responder {
    log::info!("Shutdown as requested");
    std::process::exit(0);
    format!("Exited")
}

pub fn route(config: &mut web::ServiceConfig) {
    // config.service(hello);
    
    // userHandler
    config.service(userHandler::dbtest);
    config.service(userHandler::extractor_multiple);
    config.service(userHandler::userlogin);
    config.service(userHandler::getUserInfo);
    config.service(userHandler::postTest);
    config.service(userHandler::tfaRequiredCheck);
    
    // problemHandler
    config.service(problemHandler::getProblem);
    config.service(problemHandler::getProblemTags);
        // config.service()


    // submissionHandler
    config.service(submissionHandler::submitCode);
    config.service(submissionHandler::getSubmission);
    config.service(submissionHandler::getSubmissionsList);
    config.service(submissionHandler::submissionExists);
    config.service(submissionHandler::shareSubmission);

}



//测试案例
// #[derive(Deserialize, Serialize, Clone, Default, Debug)]
// pub struct User {
//     pub name: String,
//     pub id: u32,
// }

// #[get("/hello")]
// async fn hello(req_body: String) -> impl Responder {
//     let user: User = User {
//         name: "hello".to_string(),
//         id: 32,
//     };
//     HttpResponse::Ok().body(serde_json::to_string_pretty(&user).unwrap())
//     // HttpResponse::Ok().body("hello boy")
// }
