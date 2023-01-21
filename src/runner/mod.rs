use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio, Output};
// use std::sync::Arc;
use std::time::{Duration, Instant};
use std::usize;
use actix_web::web::Data;
use chrono::format::format;
use wait_timeout::ChildExt;
use tokio::sync::Mutex;
use mysql::*;
use mysql::prelude::*;

use std::convert::TryFrom;
use std::convert::TryInto;


use self::diff::diff_strict;

use super::job;
use super::config::{*,self};
use crate::submission::{self,*};
// use crate::problem::{self, getProblemByID, getTimeByProblemId};
use crate::handler::submissionHandler::{SubmissionData,SubmissionWeb};

pub mod diff;
pub mod util;

pub fn compileForSub(
    sub: &submission::Submission,
    config: &Data<config::Config>,
)->Option<i32>{
    let _ = std::fs::create_dir("oj_runtime_dir");
    let _ = std::fs::remove_dir_all(format!("oj_runtime_dir/job_{}", sub.id));
    let _ = std::fs::create_dir(format!("oj_runtime_dir/job_{}", sub.id));
    let path = format!("oj_runtime_dir/job_{}",sub.id ).to_string();
    let mut lang = config::Language {
        ..config::Language::default()
    };

    for language in &config.languages {
        if language.name.eq(&sub.language) {
            lang.name = language.name.to_string();
            lang.file_name = language.file_name.to_string();
            lang.command = language.command.iter().map(|s| s.to_string()).collect();
            break;
        }
    }

    log::info!("compile Language: {:?}", lang);

    let mut file =
        std::fs::File::create(format!("{}/{}", path, lang.file_name)).expect("Cannot create file.");
    let _ = file.write_all(sub.code.as_bytes());

    let (mut input_index, mut output_index): (Option<usize>, Option<usize>) = (None, None);
    for (index, arg) in lang.command.iter().enumerate() {
        if arg.eq("%INPUT%") {
            input_index = Some(index);
        } else if arg.eq("%OUTPUT%") {
            output_index = Some(index);
        }
    }

    let bin_path: String = match cfg!(target_os = "windows") {
        true => format!("{}/job.exe", path).to_string(),
        false => format!("{}/job", path).to_string(),
    };

    println!("{}",bin_path);
    if input_index.is_some() {
        lang.command[input_index.unwrap()] = format!("{}/{}", path, lang.file_name);
    }
    if output_index.is_some() {
        lang.command[output_index.unwrap()] = bin_path.to_string();
    }

    let mut compiler = Command::new(&lang.command[0])
    .args(&lang.command[1..])
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .unwrap();
    let wait_time = Duration::from_secs(15); //compiling for at most 15 seconds

    
  
    let status_code = match compiler.wait_timeout(wait_time).unwrap() {
        Some(status) => status.code(),
        None => {
            compiler.kill().unwrap();
            compiler.wait().unwrap().code()
        }
    };
    // println!("status_code{}",status_code);
    status_code
}

pub fn compile(
    body: job::PostJob,
    config: Data<config::Config>,
    job_id: u32,
) {
    let _ = std::fs::create_dir("oj_runtime_dir");
    let _ = std::fs::remove_dir_all(format!("oj_runtime_dir/job_{}", job_id));
    let _ = std::fs::create_dir(format!("oj_runtime_dir/job_{}", job_id));
    let path = format!("oj_runtime_dir/job_{}", job_id).to_string();
    let mut lang = config::Language {
        ..config::Language::default()
    };

    for language in &config.languages {
        if language.name.eq(&body.language) {
            lang.name = language.name.to_string();
            lang.file_name = language.file_name.to_string();
            lang.command = language.command.iter().map(|s| s.to_string()).collect();
            break;
        }
    }

    log::info!("compile Language: {:?}", lang);

    let mut file =
        std::fs::File::create(format!("{}/{}", path, lang.file_name)).expect("Cannot create file.");
    let _ = file.write_all(body.source_code.as_bytes());

    let (mut input_index, mut output_index): (Option<usize>, Option<usize>) = (None, None);
    for (index, arg) in lang.command.iter().enumerate() {
        if arg.eq("%INPUT%") {
            input_index = Some(index);
        } else if arg.eq("%OUTPUT%") {
            output_index = Some(index);
        }
    }

    let bin_path: String = match cfg!(target_os = "windows") {
        true => format!("{}/job.exe", path).to_string(),
        false => format!("{}/job", path).to_string(),
    };

    println!("{}",bin_path);
    if input_index.is_some() {
        lang.command[input_index.unwrap()] = format!("{}/{}", path, lang.file_name);
    }
    if output_index.is_some() {
        lang.command[output_index.unwrap()] = bin_path.to_string();
    }

    let mut compiler = Command::new(&lang.command[0])
    .args(&lang.command[1..])
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .unwrap();
    let wait_time = Duration::from_secs(15); //compiling for at most 15 seconds

    // compiler.
  
    let status_code = match compiler.wait_timeout(wait_time).unwrap() {
        Some(status) => status.code(),
        None => {
            compiler.kill().unwrap();
            compiler.wait().unwrap().code()
        }
    };

}


pub fn run(index:u32,input_path:&String,bin_path:&String,
    out_path:&String,time_limit:u64,mem_limit:u64)
    ->core::result::Result<u128,i8>
{   let input_file=format!("{}/{}.in",input_path,index);
    let out_file = format!("{}/{}.out", out_path, index);
    let now = Instant::now();
    let mut runner = Command::new(&bin_path)
        .stdin(Stdio::from(std::fs::File::open(input_file).unwrap()))
        .stdout(Stdio::from(std::fs::File::create(out_file).unwrap()))
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let wait_time = Duration::from_millis(time_limit);
    let mut real_time: u128 = 0;
    match runner.wait_timeout(wait_time).unwrap() {
        Some(status) => {
            if status.code().unwrap() != 0 {
                //Runtime Error
                return Err(RESULTS::RUNTIME_ERROR);
            } else {
                //Exited Normally
                real_time = now.elapsed().as_micros();

                return Ok(real_time);
            }
        }
        None => {
            //Time Limit Exceeded
            real_time = now.elapsed().as_micros();
            return Err(RESULTS::REAL_TIME_LIMIT_EXCEEDED);
        }
    };


}
use file_diff;
pub fn score_single(index:usize, input_path:&String,out_path:&String)->bool{
    let input_file=format!("{}/{}.out",input_path,index);
    let out_file = format!("{}/{}.out", out_path, index);
    println!("{} {}",input_file,out_file);

    file_diff::diff(input_file.as_str(), out_file.as_str())
}



pub async fn judge(
    pool : &Data<Mutex<Pool>>,
    config: Data<Config>,
    body: SubmissionData,
    sub: Submission,
)->bool {
    // TODO: Need a Submission struct
    // let mut submission = SubmissionWeb::default();
    // insert submission to database

    // let username=("Durant").to_string();
    // let mut sub= match submission::createSubmission(
    //     pool,
    //     body.contest_id,
    //     body.problem_id,
    //     &username,
    //     &body.language,
    //     &body.code,
    // ).await{
    //     Some(one)=>one,
    //     _=>{
    //         Submission::default()
    //     }
    //     None => todo!(),
    // };
    // println!("{:?}",sub);

    updateResult(pool,RESULTS::JUDGING,sub.id).await;
    let status_code=compileForSub(&sub, &config);
    match status_code {
        Some(0) => {
            //Compilation Success
        }
        _ => {
            log::warn!("compile error");
            // println!("compile error");
            updateResult(pool,RESULTS::COMPILE_ERROR,sub.id).await;
            
            return false;
        }
    };
    // let cases=getCasesByProblemId(pool, sub.problem as u64);
    // // let mut score: f32 = 0.0;
    // let mut flag: bool = true;
    // let cases=match cases.await{
    //     Some(cases)=>{
    //         cases
    //     },
    //     _=>{
    //         updateResult(pool, RESULTS::ACCEPTED, sub.id);
    //         updateScore(pool,100,sub.id );
    //         log::warn!("{} problem didn't have cases, so submission all accepted",sub.problem);
    //         return false;
    //     }
    // };
    

    let id=sub.id;
    let problem=sub.problem;
    let bin_path=format!("oj_runtime_dir/job_{}/job.exe", id);
    let out_path=format!("oj_runtime_dir/job_{}/", id);
    let input_path=format!("problems/{}",problem);
 
    // let (time_limit,mem_limit)=getTimeByProblemId(problem);
    let time_limit=1000000;
    let mem_limit=1000000;
   
    let f_names=util::dirGet(&input_path, ".in");

    // let pre_score:f32=(100)as f32/ f32::try_from(f_names.len() as u32).unwrap() ;
    let mut tot=0;
    let mut pass=0;
    let mut time_tot:u64=0;
    for (index, value) in f_names.iter().enumerate(){
        let indexi=index+1;
        let res=run(indexi as u32, &input_path, &bin_path, &out_path, time_limit, mem_limit);
        tot=tot+1;
        match res {
            Ok(time)=>{
                log::info!("{} problem time is {}",problem,time);
                if score_single(indexi, &input_path, &out_path){
                    pass=pass+1;
                }
                time_tot+=time as u64;
            }
            Err(result)=>{
                log::warn!("{} problem runtime error result:{}",problem,result);
                updateResult(pool, result, id).await;
                return false;
            }
        }
    }
    

    let score=pass * 100 / tot; 
    if(tot==pass){
        updateResult(pool,RESULTS::ACCEPTED , id).await;
    }else{
        updateResult(pool, RESULTS::WRONG_ANSWER, id).await;
    }
    true
}



pub async fn judgeForSub(
    pool : &Data<Mutex<Pool>>,
    config: Data<Config>,
    sub: Submission,
)->bool{
    updateResult(pool,RESULTS::JUDGING,sub.id).await;
    let status_code=compileForSub(&sub, &config);
    match status_code {
        Some(0) => {
            //Compilation Success
        }
        _ => {
            log::warn!("compile error");
            // println!("compile error");
            updateResult(pool,RESULTS::COMPILE_ERROR,sub.id).await;
            
            return false;
        }
    };
    
    let id=sub.id;
    let problem=sub.problem;
    let bin_path=format!("oj_runtime_dir/job_{}/job.exe", id);
    let out_path=format!("oj_runtime_dir/job_{}/", id);
    let input_path=format!("problems/{}",problem);
 
    // let (time_limit,mem_limit)=getTimeByProblemId(problem);
    let time_limit=1000000;
    let mem_limit=1000000;
   
    let f_names=util::dirGet(&input_path, ".in");

    // let pre_score:f32=(100)as f32/ f32::try_from(f_names.len() as u32).unwrap() ;
    let mut tot=0;
    let mut pass=0;
    let mut time_tot:u64=0;
    for (index, value) in f_names.iter().enumerate(){
        let indexi=index+1;
        let res=run(indexi as u32, &input_path, &bin_path, &out_path, time_limit, mem_limit);
        tot=tot+1;
        match res {
            Ok(time)=>{
                log::info!("{} problem time is {}",problem,time);
                if score_single(indexi, &input_path, &out_path){
                    pass=pass+1;
                }
                time_tot+=time as u64;
            }
            Err(result)=>{
                log::warn!("{} problem runtime error result:{}",problem,result);
                updateResult(pool, result, id).await;
                return false;
            }
        }
    }
    

    let score=pass * 100 / tot; 
    if(tot==pass){
        updateResult(pool,RESULTS::ACCEPTED , id).await;
    }else{
        updateResult(pool, RESULTS::WRONG_ANSWER, id).await;
    }
    true

}