use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};
// use std::sync::Arc;
use std::time::{Duration, Instant};
use actix_web::web::Data;
use wait_timeout::ChildExt;
use tokio::sync::Mutex;
use mysql::*;
use mysql::prelude::*;



use super::job;
use super::config::{*,self};
use crate::submission::{self,*};
use crate::problem::{self, getProblemByID, getCasesByProblemId};
use crate::handler::submissionHandler::{SubmissionData,SubmissionWeb};

mod diff;

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

    // compiler.
  
    let status_code = match compiler.wait_timeout(wait_time).unwrap() {
        Some(status) => status.code(),
        None => {
            compiler.kill().unwrap();
            compiler.wait().unwrap().code()
        }
    };
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


pub async fn judge(
    pool : &Data<Mutex<Pool>>,
    config: Data<Config>,
    body: SubmissionData,
)->bool {
    // TODO: Need a Submission struct
    let mut submission = SubmissionWeb::default();
    // insert submission to database

    let username=("username").to_string();
    let mut sub= match submission::createSubmission(
        pool,
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
        None => todo!(),
    };

    updateResult(pool,RESULTS::JUDGING,sub.id);
    let status_code=compileForSub(&sub, &config);
    match status_code {
        Some(0) => {
            //Compilation Success
        }
        _ => {
            updateResult(pool,RESULTS::COMPILE_ERROR,sub.id);
            return false;
        }
    };
    let cases=getCasesByProblemId(pool, sub.problem as u64);
    match cases.await{
        Some(cases)=>{
            

        },
        _=>{
            
        }
    }



    
    true
}