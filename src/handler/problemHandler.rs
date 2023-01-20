use core::borrow;
use std::collections::HashMap;

use actix_web::http::header::ContentType;
use actix_web::{delete, get, post, web,web::Data};
use actix_web::{HttpResponse, Responder};
use chrono::DateTime;
use mysql::binlog::jsonb::Array;
use serde::{Deserialize, Serialize};
use serde_json;
use super::super::config::Config;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;
// use derivative::Derivative;

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

// #[derive(Derivative)]
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ProblemIOMode {
    standard: String,

}


#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Problem {
    _id: u64,
    contest: String,
    is_public: bool,
    title: String,
    description: String,
    // input_description: String,
    // output_description: String,
    // samples: Sample,
    // test_case_id: String,
    // test_case_score: TestCaseScore, 
    // hint: String,
    // languages: String,
    // template: String,
    // create_time: String,
    // last_update_time: String,
    // created_by: String,
    // time_limit: u64,
    // memory_limit: u64,
    // io_mode: 

}


#[get("/api/problem")]
async fn getProblem ( 
    // body: web::Json<PostJob>,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{

    let mut conn=pool.lock().await.get_conn().unwrap();// 获取链接
    let mut problem: Problem = Problem { 
        _id: 0, 
        contest: "".to_string(),
        is_public: true,
        title: "测试题目".to_string(),
        description: "测试描述".to_string()
    };

    // conn.query_iter("select * from `tb_problem`")
    //     .unwrap()
    //     .for_each(|row| {
    //         let r:(u64, String, String) = from_row(row.unwrap());
    //         log::info!("查询题库{}, {}, {}", r.0, r.1, r.2);
    //         problem.problem_id=r.0;
    //         problem.problem_title=r.1;
    //         problem.problem_path=r.2;
    // });

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&problem).unwrap());

}

#[get("/api/problem/tags")]
async fn getProblemTags (
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
) -> impl Responder {
    let mut conn = pool.lock().await.get_conn().unwrap();
    let mut tags: ProblemTag = ProblemTag { name: "无标签".to_string() };

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&tags).unwrap());
}
