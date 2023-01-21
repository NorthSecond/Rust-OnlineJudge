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

pub struct Case{
    input:String,
    ouput:String,
}

pub struct ProblemTi {
    pub _id: u64,
    pub problemTitle: String,
    pub cases: Vec<Case>,
}

pub async fn getCasesByProblemId(
    pool: &web::Data<Mutex<Pool>>,
    id:u64)->Option<Vec<Case>> {
    None
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

pub async fn getProblemListWithOffsite (
    pool: web::Data<Mutex<Pool>>,
    offsite: u64,
    limit: u64
)->Option<Vec<Problem>> {
    let RProblems=
        getProblems(pool, &format!("where `problem_id` between {} and {}", offsite * limit, offsite * limit + limit)).await;

    match RProblems {
        Ok(problems)=>Some(problems),
        Err(info)=>None,
    }
}
