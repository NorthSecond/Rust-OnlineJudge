use actix_web::middleware::Condition;
use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;

use crate::config::{Problem, Language};


mod RESULTS{
    pub const COMPILE_ERROR:i8 = -2;
    pub const WRONG_ANSWER:i8 = -1;
    pub const ACCEPTED:i8 = 0;
    pub const CPU_TIME_LIMIT_EXCEEDED:i8 = 1;
    pub const REAL_TIME_LIMIT_EXCEEDED:i8 = 2;
    pub const MEMORY_LIMIT_EXCEEDED:i8 = 3;
    pub const RUNTIME_ERROR:i8 = 4;
    pub const SYSTEM_ERROR:i8 = 5;
    pub const PENDING:i8 = 6;
    pub const JUDGING:i8 = 7;
    pub const PARTIALLY_ACCEPTED:i8 = 8;
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Statistic_info{
    pub time_cost:u32,
    pub memory_cost:u32,
    pub err_info:String,
    pub score:u8,  //百分
}

#[derive (Default,Clone,Debug)]
pub struct Submission{
   pub id:u32,
   pub contest:u32,
   pub problem:u32,
   pub language:String,
   pub username:String,
   pub code:String,
   pub create_time:String,
   pub result:i8,
   pub time_cost:u32,
   pub memory_cost:u32,
   pub err_info:String,
   pub score:u8,  //百分
//    {time_cost: "", memory_cost: "", err_info: "", score: 0}
//    pub ip:String
}

impl Submission {
    pub fn new(
        contest:u32,
        problem:u32,
        language:String,
        username:String,
        code:String,
    )->Submission{
        Submission{
            id:0,
            contest:contest,
            problem:problem,
            create_time:"2023-01-20 20:46:00".to_string(),
            language:language,
            username:username,
            code:code,
            result:RESULTS::PENDING,
            time_cost: 0, 
            memory_cost: 0, 
            err_info: "".to_string(), 
            score: 0 
        }
    }

}


pub async fn get(
    pool: web::Data<Mutex<Pool>>,
    condition:&String
) -> Result<Vec<Submission>> {
    let mut conn=pool.lock().await.get_conn().unwrap();
    let users=conn.query_map(
        format!("select * from tb_submission {}",condition), 
        |(   
            id,
            contest,
            problem,
            create_time ,
            username ,
            language,
            code ,
            result ,
            time_cost   ,
            memory_cost ,
            err_info   ,
            score ,
        )|
        { 
            log::info!("{}",create_time);
            Submission { id: id, contest: contest, problem: problem, create_time:create_time, username: username, language:language ,code: code, result:result, time_cost: time_cost, memory_cost:memory_cost, err_info: err_info, score:score }
        },
    );
    users
}

pub async fn getById(
    pool: web::Data<Mutex<Pool>>,
    id:u32
)->Option<Submission>{
    let Robjs=
    get(pool, &format!("where id={}",id)).await;

    match Robjs{
        Ok(objs)=>objs.get(0).cloned(),
        Err(info)=>None,
    }
}

pub async fn createSubmission(
    pool: web::Data<Mutex<Pool>>,
    contest:u32,
    problem:u32,
    username:String,
    language:String,
    code:String,
) ->Result<()> {
    log::info!("Create Summission...");
    log::info!("contest:{}, problem:{}, username:{}", contest, problem, username);
    let mut conn=pool.lock().await.get_conn().unwrap();
    // 检查username是否存在

    let r=conn.exec_drop(
        "insert into `tb_submission`(`id`,`contest`, `problem`, `username`,`language`,`code`,`result`,`create_time`) values (null,:c, :p, :u,:lang,:code,:r,now());", 
        params!{
                "c" => contest, 
                "p" => problem, 
                "u" => username,
                "lang"=>language,
                "code"=>code,
                "r" => RESULTS::PENDING,
    });
    r
}

















