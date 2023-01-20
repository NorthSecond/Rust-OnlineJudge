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