use super::super::config::Config;
use crate::problem::{self, *};
use actix_web::http::header::ContentType;
use actix_web::{delete, get, post, web,  web::Data};
use actix_web::{HttpResponse, Responder};
use chrono::DateTime;
use core::borrow;
use mysql::binlog::jsonb::Array;
use mysql::prelude::*;
use mysql::*;
use mysql::prelude::*;
use mysql::*;
use serde::__private::de;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ProblemTag {
    name: String,
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
    samples: Vec<Sample>,
    test_case_id: String,
    test_case_score: TestCaseScore,
    hint: String,
    languages: Vec<String>,
    template: String,
    create_time: String,
    last_update_time: String,
    created_by: String,
    time_limit: f32,
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
async fn getProblemList(
    pool: Data<Mutex<Pool>>,
    config: Data<Config>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    log::info!("调用题目列表！");
    let mut limit = query
        .get("limit")
        .unwrap_or(&"10".to_string())
        .parse::<u64>()
        .unwrap();
    let mut offsite = query
        .get("offsite")
        .unwrap_or(&"0".to_string())
        .parse::<u64>()
        .unwrap();
    let mut id = query
        .get("problem_id")
        .unwrap_or(&"0".to_string())
        .parse::<u64>()
        .unwrap();

    // limit + offsite是一个接口
    // ploblem id 是一个接口

    if id != 0 {
        let mut p = Detailed_Problem::default();
        match getProblemByID(pool, id).await {
            Some(problem) => {
                // detailed_problem._id = problem.problemID.clone(),
                let path = format!("./problems/{}/problem.json", id);
                let _path = Path::new(path.as_str());
                let problem_file = File::open(_path).unwrap();
                let content: serde_json::Value = serde_json::from_reader(problem_file).unwrap();

                p._id = problem._id.clone();
                p.title = problem.problemTitle.clone();
                let mut description = content["description"].to_string();
                // delete the first and last "
                description = description[1..description.len() - 1].to_string();
                p.description = description.replace("\\r", "\r").replace("\\n", "\n").replace("\\\"", "\"").replace("\\t", "\t");


                p.time_limit = content["time_limit"]["value"]
                    .to_string()
                    .parse::<f32>()
                    .unwrap();
                p.memory_limit = content["memory_limit"]["value"]
                    .to_string()
                    .parse::<u64>()
                    .unwrap();
                p.samples = vec![];
                let mut sample = Sample::default();
                let mut input = content["samples"][0]["input"].to_string();
                // delete the first and last "
                input = input[1..input.len() - 1].to_string();
                sample.input = input.replace("\\r", "\r").replace("\\n", "\n").replace("\\\"", "\"").replace("\\t", "\t");
                let mut output = content["samples"][0]["output"].to_string();
                // delete the first and last "
                output = output[1..output.len() - 1].to_string();
                sample.output = output.replace("\\r", "\r").replace("\\n", "\n").replace("\\\"", "\"").replace("\\t", "\t");
                p.samples.push(sample);
                let mut hint = content["hint"].to_string();
                if hint == "null" {
                    hint = "".to_string();
                }
                p.hint = hint;
                p.source = content["source"].to_string();
                p.difficulty = "Mid".to_string();
                p.statistic_info = 0;
                p.languages = vec![
                    "C".to_string(),
                    "C++".to_string(),
                    "Rust".to_string(),
                ];
                let problemRes = ProblemRes {
                    data: p,
                    error: "".to_string(),
                };
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string_pretty(&problemRes).unwrap());
            }
            None => {
                let problemRes = ProblemRes {
                    data: p,
                    error: "Problem does not exist".to_string(),
                };
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string_pretty(&problemRes).unwrap());
            }
        }
    } else {
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

        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string_pretty(&problemListRes).unwrap());
    }
}

#[get("/api/problem/tags")]
async fn getProblemTags(pool: Data<Mutex<Pool>>, config: Data<Config>) -> impl Responder {
    let mut conn = pool.lock().await.get_conn().unwrap();
    let mut tag: ProblemTag = ProblemTag {
        name: "无标签".to_string(),
    };
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
