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
use std::fmt::format;
use actix_web::http::header::ContentType;

use crate::runner::*;
use crate::submission::{*,self, RESULTS};
use crate::error_log::SUBMISSION;

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
    problem_id: u32,
    language: String,
    code: String,
    contest_id: u32,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct SubmissionWeb {
    id: u32,
    contest: u32,
    problem: u32,
    create_time: String,
    user_id: String,
    code : String,
    result: i8,
    info : SubmissionInfo,
    language: String,
    shared: bool,
    statistic_info: String,
    ip : String,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct SubmissionInfo {
    time_cost: u32,
    memory_cost: u32,
    err_info: String,
    result: i8,
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

    let username=("username").to_string();
    let mut sub= match createSubmission(
        &pool,
        body.contest_id,
        body.problem_id,
        &username,
        &body.language,
        &body.code,
    ).await{
        Some(one)=>one,
        _=>{
            
            Submission::default()
        }
    };



    update(pool, format!("result = {}",RESULTS::JUDGING),format!("id = {}",sub.id));
    compileForSub(&sub, &config);


    // compile(body, config, sub.id);
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
    let mut id = query.get("id").unwrap_or(&"10".to_string()).parse::<u32>().unwrap();
    
    // should be a Submission
    let submissionOpt = getById(&pool, id).await;

    let submission = match submissionOpt{
        Some(submission) => submission,
        None => Submission::default(),
    };

    let mut info :SubmissionInfo = SubmissionInfo::default();
    info.time_cost = submission.time_cost;
    info.memory_cost = submission.memory_cost;
    info.err_info = submission.err_info;
    info.result = submission.result;

    let mut submissionWeb = SubmissionWeb::default();
    submissionWeb.id = submission.id;
    submissionWeb.contest = submission.contest;
    submissionWeb.problem = submission.problem;
    submissionWeb.create_time = submission.create_time;
    submissionWeb.user_id = submission.username;
    submissionWeb.code = submission.code;
    submissionWeb.result = submission.result;
    submissionWeb.info = info;
    submissionWeb.language = submission.language;
    submissionWeb.shared = false;
    submissionWeb.statistic_info = String::from("");
    submissionWeb.ip = String::from("");
    
    // get submission from database

    // return result
    HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&submissionWeb).unwrap())
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
    
    let mut submissions = match get(&pool, &format!("1")).await {
        Ok(submissions) => submissions,
        Err(_) => Vec::new(),
    };

    // 切片
    submissions = submissions.into_iter().skip(offsite as usize).take(limit as usize).collect::<Vec<Submission>>();

    let mut submissionsWeb : Vec<SubmissionWeb> = Vec::new();

    for submission in submissions{
        let mut info :SubmissionInfo = SubmissionInfo::default();
        info.time_cost = submission.time_cost;
        info.memory_cost = submission.memory_cost;
        info.err_info = submission.err_info;
        info.result = submission.result;

        let mut submissionWeb = SubmissionWeb::default();
        submissionWeb.id = submission.id;
        submissionWeb.contest = submission.contest;
        submissionWeb.problem = submission.problem;
        submissionWeb.create_time = submission.create_time;
        submissionWeb.user_id = submission.username;
        submissionWeb.code = submission.code;
        submissionWeb.result = submission.result;
        submissionWeb.info = info;
        submissionWeb.language = submission.language;
        submissionWeb.shared = false;
        submissionWeb.statistic_info = String::from("");
        submissionWeb.ip = String::from("");

        submissionsWeb.push(submissionWeb);
    };
    
    // get submissions from database

    // return result
    HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&submissionsWeb).unwrap())
}

#[get("/api/submission_exists")]
async fn submissionExists(
    pool : web::Data<Mutex<Pool>>,
    config: web::Data<Config>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let mut problem_id = query.get("problem_id").unwrap_or(&"10".to_string()).parse::<u32>().unwrap();
    // get submissions from database
    
    let mut submissions = getByProblemId(pool, problem_id).await;
    
    match submissions{
        Ok(submissions) => {
            if submissions.len() > 0 {
                HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&true).unwrap())
            } else {
                HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&false).unwrap())
            }
        },
        Err(_) => {
            HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&false).unwrap())
        }
    }
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
