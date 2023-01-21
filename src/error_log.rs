use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;



#[derive(Deserialize, Serialize)]
pub struct LOGIN {
    error: u32,
    data: &'static str,
    message: String,
}

impl LOGIN  {
    pub fn ID_NO_FOUND(message: &str) ->HttpResponse{
        HttpResponse::Ok().json(LOGIN{
            error:1,
            data:"username no found",
            message:message.to_string(),
        })
    }
    pub fn PASSWORD_ERROR(message: &str)-> HttpResponse{
        HttpResponse::Ok().json(LOGIN{
            error:2,
            data:"password error",
            message:message.to_string(),
        })
    }
    pub fn NO_LOGIN(message: &str)-> HttpResponse{
        HttpResponse::Ok().json(LOGIN{
            error:3,
            data:"Please login",
            message:message.to_string(),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct NOT_FOUND {
    code: u32,
    reason: &'static str,
    message: String,
}

impl NOT_FOUND {
    pub fn new(message: &str) -> NOT_FOUND {
        NOT_FOUND {
            code: 3,
            reason: "ERR_NOT_FOUND",
            message: message.to_string(),
        }
    }
    pub fn msg(message: &str) -> String {
        to_string_pretty(&NOT_FOUND::new(message)).unwrap()
    }
    pub fn webmsg(message: &str) -> HttpResponse {
        HttpResponse::NotFound().body(NOT_FOUND::msg(message))
    }
}
pub struct SUBMISSION {
    error: u32,
    data: &'static str,
    message: String,
}

impl  SUBMISSION {
    pub fn SUBMIT_FAILE(message: &str)-> HttpResponse{
        HttpResponse::Ok().json(LOGIN{
            error:4,
            data:"submit failed",
            message:message.to_string(),
        })
    }
}
