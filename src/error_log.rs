use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Deserialize, Serialize)]
pub struct INVALID_ARGUMENT {
    code: u32,
    reason: &'static str,
    message: String,
}

impl INVALID_ARGUMENT {
    pub fn new(message: &str) -> INVALID_ARGUMENT {
        INVALID_ARGUMENT {
            code: 1,
            reason: "ERR_INVALID_ARGUMENT",
            message: message.to_string(),
        }
    }
    pub fn msg(message: &str) -> String {
        to_string_pretty(&INVALID_ARGUMENT::new(message)).unwrap()
    }
    pub fn webmsg(message: &str) -> HttpResponse {
        HttpResponse::BadRequest().body(INVALID_ARGUMENT::msg(message))
    }
}

#[derive(Deserialize, Serialize)]
pub struct INVALID_STATE {
    code: u32,
    reason: &'static str,
    message: String,
}

impl INVALID_STATE {
    pub fn new(message: &str) -> INVALID_STATE {
        INVALID_STATE {
            code: 2,
            reason: "ERR_INVALID_STATE",
            message: message.to_string(),
        }
    }
    pub fn msg(message: &str) -> String {
        to_string_pretty(&INVALID_STATE::new(message)).unwrap()
    }
    pub fn webmsg(message: &str) -> HttpResponse {
        HttpResponse::BadRequest().body(INVALID_STATE::msg(message))
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

#[derive(Deserialize, Serialize)]
pub struct RATE_LIMIT {
    code: u32,
    reason: &'static str,
    message: String,
}

impl RATE_LIMIT {
    pub fn new(message: &str) -> RATE_LIMIT {
        RATE_LIMIT {
            code: 4,
            reason: "ERR_RATE_LIMIT",
            message: message.to_string(),
        }
    }
    pub fn msg(message: &str) -> String {
        to_string_pretty(&RATE_LIMIT::new(message)).unwrap()
    }
    pub fn webmsg(message: &str) -> HttpResponse {
        HttpResponse::BadRequest().body(RATE_LIMIT::msg(message))
    }
}

#[derive(Deserialize, Serialize)]
pub struct EXTERNAL {
    code: u32,
    reason: &'static str,
    message: String,
}

impl EXTERNAL {
    pub fn new(message: &str) -> EXTERNAL {
        EXTERNAL {
            code: 5,
            reason: "ERR_EXTERNAL",
            message: message.to_string(),
        }
    }
    pub fn msg(message: &str) -> String {
        to_string_pretty(&EXTERNAL::new(message)).unwrap()
    }
    pub fn webmsg(message: &str) -> HttpResponse {
        HttpResponse::InternalServerError().body(EXTERNAL::msg(message))
    }
}

#[derive(Deserialize, Serialize)]
pub struct INTERNAL {
    code: u32,
    reason: &'static str,
    message: String,
}

impl INTERNAL {
    pub fn new(message: &str) -> INTERNAL {
        INTERNAL {
            code: 6,
            reason: "ERR_INTERNAL",
            message: message.to_string(),
        }
    }
    pub fn msg(message: &str) -> String {
        to_string_pretty(&INTERNAL::new(message)).unwrap()
    }
    pub fn webmsg(message: &str) -> HttpResponse {
        HttpResponse::InternalServerError().body(INTERNAL::msg(message))
    }
}