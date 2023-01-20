// submission handler

use actix_web::{web, HttpResponse, Responder, post, get, put};
use serde::{Deserialize, Serialize};
use super::super::config::Config;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;
use chrono::DateTime;
use serde_json;
use std::collections::HashMap;
use actix_web::http::header::ContentType;

use crate::runner::*;
use crate::submission::*;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct SubmissionData{
    /*
            let data = {
          problem_id: this.problem.id,
          language: this.language,
          code: this.code,
          contest_id: this.contestID
        }
    */
    problem_id: u64,
    language: String,
    code: String,
    contest_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct SubmissionWeb {
    id: u64,
    contest: u64,
    problem: u64,
    create_time: String,
    user_id: u64,
    code : String,
    result: i32,
    info : SubmissionInfo,
    language: String,
    shared: bool,
    statistic_info: String,
    ip : String,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct SubmissionInfo {
    time_cost: String,
    memory_cost: String,
    err_info: String,
    result: i32,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct SubmissionBrief {
    //        let data = {id: this.submission.id, shared: shared}
    id: u32,
    shared: bool,
}

/*
class JudgeStatus:
    COMPILE_ERROR = -2
    WRONG_ANSWER = -1
    ACCEPTED = 0
    CPU_TIME_LIMIT_EXCEEDED = 1
    REAL_TIME_LIMIT_EXCEEDED = 2
    MEMORY_LIMIT_EXCEEDED = 3
    RUNTIME_ERROR = 4
    SYSTEM_ERROR = 5
    PENDING = 6
    JUDGING = 7
    PARTIALLY_ACCEPTED = 8
*/

#[post("/api/submission")]
async fn submitCode(
    body: web::Json<SubmissionData>,
    pool : web::Data<Mutex<Pool>>,
    config: web::Data<Config>,
) -> impl Responder {

    // call judge
    
    // TODO: Need a Submission struct
    let mut submission = SubmissionWeb::default();
    // insert submission to database

    // return result
    HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&submission).unwrap())
}

#[get("/api/submission")]
async fn getSubmission(
    pool : web::Data<Mutex<Pool>>,
    config: web::Data<Config>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    // get submission from database
    let mut id = query.get("id").unwrap_or(&"10".to_string()).parse::<u64>().unwrap();
    
    // should be a Submission
    let mut submission = SubmissionWeb::default();
    
    // get submission from database

    // return result
    HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&submission).unwrap())
}

#[get("/api/submissions")]
async fn getSubmissionsList(
    pool : web::Data<Mutex<Pool>>,
    config: web::Data<Config>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    // get submissions from database
    let mut limit = query.get("limit").unwrap_or(&"10".to_string()).parse::<u64>().unwrap();
    let mut offsite = query.get("offsite").unwrap_or(&"0".to_string()).parse::<u64>().unwrap();
    
    // should be a list of Submission
    let mut submissions: Vec<SubmissionWeb> = Vec::new();
    
    // get submissions from database

    // return result
    HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&submissions).unwrap())
}

#[get("/api/submission_exists")]
async fn submissionExists(
    pool : web::Data<Mutex<Pool>>,
    config: web::Data<Config>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    // get submissions from database
    let mut problem_id = query.get("limit").unwrap_or(&"10".to_string()).parse::<u64>().unwrap();
    


    HttpResponse::Ok().content_type(ContentType::json()).body("ok")
}

#[put("/api/submission")]
async fn shareSubmission(
    body: web::Data<SubmissionBrief>,
    pool : web::Data<Mutex<Pool>>,
    config: web::Data<Config>,
) -> impl Responder {
    // get id from body
    //    let data = {id: this.submission.id, shared: shared}
    let id = body.id;
    let shared = body.shared;
     
    // db operation
    // 但是现在还没有提交表，所以先不写了
    // let mut conn=pool.lock().await.get_conn().unwrap();// 获取链接
    // let mut sql = format!("update submission set shared = {} where id = {}", shared, id);

    // let result = conn.exec_drop(sql).unwrap();
    // return result
    HttpResponse::Ok().content_type(ContentType::json()).body("ok")
}
