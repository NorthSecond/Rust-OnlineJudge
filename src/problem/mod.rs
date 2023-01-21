use actix_web::middleware::Condition;
use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;

#[derive(Clone,Default)]
pub struct Problem {
    pub _id: u64,
    pub problemTitle: String,
    pub problemPath: String,
}

pub async fn getProblems (
    pool: web::Data<Mutex<Pool>>,
    condition:&String
) -> Result<Vec<Problem>> {
    let mut conn=pool.lock().await.get_conn().unwrap();
    let problems=conn.query_map(
        format!("select * from tb_problem {}",condition), 
        |(problem_id, problem_title, problem_path)|
        { 
            Problem {
                _id: problem_id,
                problemTitle: problem_title,
                problemPath: problem_path
            }
        },
    );
    problems
}

pub async fn getProblemByID (
    pool: web::Data<Mutex<Pool>>,
    _id: u64
)->Option<Problem> {
    let RProblems=
        getProblems(pool, &format!("where problem_id='{}'", _id)).await;

    match RProblems {
        Ok(problems)=>problems.get(0).cloned(),
        Err(info)=>None,
    }
}