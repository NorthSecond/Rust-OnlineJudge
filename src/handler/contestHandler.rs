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
use crate::contest;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PostContest {
    pub id: Option<u32>,
    pub name: String,
    pub from: String,
    pub to: String,
    pub problem_ids: Vec<u32>,
    pub user_ids: Vec<u32>,
    pub submission_limit: u32,
}

#[derive(Debug, Default, Serialize, Deserialize,Clone)]
pub struct PageNum{
    pub offset: i32,
    pub limit: i32,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Contest {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub real_time_rank: bool,
    pub passwork: String,
    pub rule_type: String,
    pub start_time: String,
    pub end_time: String,
    pub last_update_time: String,
    pub visible: bool,
    pub status:  String,
    pub created_by: String,
}


#[get("/api/contests")]
async fn getContestList ( 
    page: web::Query<PageNum>,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{
    let mut offsetNum = 0;
    log::info!("GetContests");
    let mut conn=pool.lock().await.get_conn().unwrap();// 获取链接
    let mut contestsNew = contest::get_contests(pool).await.unwrap();
    let mut contests: Vec<Contest> = Vec::new();
    let mut value: Contest = Contest { 
        id:  2,
        title: "测试题目".to_string(),
        description: "测试描述".to_string(),
        real_time_rank: true,
        passwork: "123".to_string(),
        rule_type: "ACM".to_string(),
        start_time: "2022-01-02".to_string(),
        end_time:   "2022-01-22".to_string(),
        last_update_time: "2022-01-14".to_string(),
        visible:     true,
        status:      '1'.to_string(),
        created_by:  "Aimi".to_string()
    };
    contestsNew.push(value);
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
    .body(serde_json::to_string(&contestsNew).unwrap());
    

}

#[get("/api/contest")]
async fn getContest ( 
    page: web::Query<PageNum>,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{
    let mut offsetNum = 0;
    log::info!("GetContests");
    let mut conn=pool.lock().await.get_conn().unwrap();// 获取链接
    let mut contestsNew = contest::get_contests(pool).await.unwrap();
    let mut contests: Vec<Contest> = Vec::new();
    let mut value: Contest = Contest { 
        id:    2,
        title: "测试题目".to_string(),
        description: "测试描述".to_string(),
        real_time_rank: true,
        passwork: "123".to_string(),
        rule_type: "ACM".to_string(),
        start_time: "2022-01-02".to_string(),
        end_time:   "2022-01-22".to_string(),
        last_update_time: "2022-01-14".to_string(),
        visible:     true,
        status:      '1'.to_string(),
        created_by:  "Aimi".to_string()
    };
    contestsNew.push(value);
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
    .body(serde_json::to_string(&contestsNew).unwrap());
    

}