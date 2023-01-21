use core::borrow;
use std::collections::HashMap;
use std::fs::File;
use actix_web::http::header::ContentType;
use actix_web::{delete, get, post, web,web::Data};
use actix_web::{HttpResponse, Responder};
use chrono::DateTime;
use mysql::binlog::jsonb::Array;
use serde::__private::de;
use serde::{Deserialize, Serialize};
use serde_json;
use super::super::config::Config;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;
use crate::problem::{*, self};
use serde_json::{Value};
use std::path::Path;


#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ProblemTag {
    name: String
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Sample {
    input: String,
    output: String,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct TestCaseScore {
    input_name: String,
    output_name: String,
    score: u64,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ProblemIOMode {
    mode: String,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct gotProblem {
    ID: u64,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Detailed_Problem {
    _id: u64,
    contest: String,
    is_public: bool,
    title: String,
    description: String,
    input_description: String,
    output_description: String,
    samples: Sample,
    test_case_id: String,
    test_case_score: TestCaseScore, 
    hint: String,
    languages: String,
    template: String,
    create_time: String,
    last_update_time: String,
    created_by: String,
    time_limit: u64,
    memory_limit: u64,
    io_mode: ProblemIOMode,
    spj: bool,
    spj_language: String,
    spj_code: String,
    spj_version: String,
    spj_compile_ok: bool,
    rule_type: String,
    difficulty: String,
    tags: ProblemTag,
    source: String,
    total_score: u64,
    submission_number: u64,
    accepted_number: u64,
    statistic_info: u64,
    share_submission: u64,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ProblemList {
    total: u64,
    results: Vec<Detailed_Problem>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ProblemListRes {
    data: ProblemList,
    error: String,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct tagRes {
    data: Vec<ProblemTag>,
    error: String,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ProblemRes {
    data: Detailed_Problem,
    error: String,
}

#[get("/api/problem")]
async fn getProblemList ( 
    pool: Data<Mutex<Pool>>,
    config: Data<Config>,
    query: web::Query<HashMap<String, String>>
)-> impl Responder{
    log::info!("调用题目列表！");
    let mut limit = query.get("limit").unwrap_or(&"10".to_string()).parse::<u64>().unwrap();
    let mut offsite = query.get("offsite").unwrap_or(&"0".to_string()).parse::<u64>().unwrap();
    // let mut conn=pool.lock().await.get_conn().unwrap();// 获取链接
    
        // get problems from database
        // offiste->limit + offsite
    let mut r: Vec<Problem> = match getProblemListWithOffsite(pool, offsite, limit).await {
        Some(r) => r,
        None => Vec::new(),
    };

    let mut res: Vec<Detailed_Problem> = Vec::new();
    for i in 0..r.len() {
        let mut tmp = Detailed_Problem::default();
        tmp._id = r[i]._id;
        tmp.title = r[i].problemTitle.clone();
        tmp.difficulty = "Mid".to_string();
        res.push(tmp);
    }

    let mut problemList = ProblemList {
        total: 20 as u64,
        results: res,
    };
    let mut problemListRes = ProblemListRes {
        data: problemList,
        error: "".to_string(),
    };
    // 需要数据查询操作

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&problemListRes).unwrap());
}

#[get("/api/problem/tags")]
async fn getProblemTags (
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
) -> impl Responder {
    let mut conn = pool.lock().await.get_conn().unwrap();
    let mut tag: ProblemTag = ProblemTag { name: "无标签".to_string() };
    let mut tags: Vec<ProblemTag> = Vec::new();
    tags.push(tag);

    let mut tagRes = tagRes {
        data: tags,
        error: "".to_string(),
    };

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&tagRes).unwrap());
}

#[get("/api/problem")]
async fn getProblem (
    pool: Data<Mutex<Pool>>,
    config: Data<Config>,
    query: web::Query<HashMap<String, String>>
) -> impl Responder {
    let mut id = query.get("_id").unwrap_or(&"10".to_string()).parse::<u64>().unwrap();
    let mut p = Detailed_Problem::default();
    match getProblemByID(pool, id).await {
        Some(problem) => {
            // detailed_problem._id = problem.problemID.clone(),
            p._id = problem._id.clone();
            p.title = problem.problemTitle.clone();
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string_pretty(&p).unwrap());
        },
        None => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string_pretty(&p).unwrap());
        }
    }
}



#[get("/api/problem/{problem_id}")]
async fn getProblemContent (
    pool: Data<Mutex<Pool>>,
    config: Data<Config>,
    query: web::Query<HashMap<String, String>>
) -> impl Responder {
    let mut id = query.get("_id").unwrap_or(&"10".to_string()).parse::<u64>().unwrap();
    let mut p = Detailed_Problem::default();
    match getProblemByID(pool, id).await {
        Some(problem) => {
            let path = format!("../../../problems/{}/problem.json",id);
            let path = Path::new(path.as_str());
            let problem_file = File::open("path").unwrap();
            let content: serde_json::Value = serde_json::from_reader(problem_file).unwrap();
            
            p._id = problem._id.clone();
            p.title = problem.problemTitle.clone();
            p.description = content["description"].to_string();
            p.time_limit = content["time_limit"].as_u64().unwrap();
            p.memory_limit = content["memory_limit"].as_u64().unwrap();
            p.samples.input = content["samples"]["input"].to_string();
            p.samples.output = content["samples"]["output"].to_string();
            p.hint = content["hint"].to_string();
            p.source = content["source"].to_string();
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string_pretty(&p).unwrap());
        },
        None => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string_pretty(&p).unwrap());
        }
    }
}



fn read_json(raw_json:&str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed
}