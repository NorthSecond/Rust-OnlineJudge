use actix_rt::time;
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

pub struct Limit {
    time_limit:u64,
    memeory_limit:u64,
}

use std::path::Path;
use std::fs::{File};

use std::convert::TryFrom;
use std::convert::TryInto;
// use std::convert::FloatToInt;
// pub  fn getTimeByProblemId(id:u32)->(u64,u64){
//     let path = format!("./problems/{}/problem.json", id);
//     let _path = Path::new(path.as_str());
//     let problem_file =match File::open(_path){
//         Ok(F)=>F,
//         Err(e)=>{
//             return (
//                 1000000000,
//                 1000000000,
//             )
//         }
//     };
//     let content: serde_json::Value = serde_json::from_reader(problem_file).unwrap();
//     let time_limit = content["time_limit"]["value"]
//         .to_string()
//         .parse::<f32>()
//         .unwrap();
//     let memory_limit = content["memory_limit"]["value"]
//         .to_string()
//         .parse::<u64>()
//         .unwrap();

//     let time_limit=time_limit as u64;
       
//     return (
//         time_limit,
//         memory_limit,
//     );

//     // let time_limit=(time_limit*(1000 as f32)).ceil().to_value();
//     // match time_limit {
//     //     Ok(time)=> {
//     //         return (
//     //            time,
//     //            memory_limit,
//     //        );
//     //     }
//     //     Err(E)=>{
//     //         return (
//     //             1000000000,
//     //             memory_limit,
//     //         )
//     //     }
//     // }   
// }

 


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
