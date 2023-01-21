mod user;
mod runner;
mod config;
mod error_log;
mod job;
mod handler;
mod submission;
mod contest;
mod problem;


#[cfg(test)]
mod runnerTest {
    use std::fs;
    use crate::{job::PostJob, runner::{self, judge}, problem, handler};

    // use std::io::BufReader;
    use super::runner::compile;
    use crate::job;
    use super::config;
    use actix_web::web::Data;
    #[test]
    fn compilefile() {

        let contents=fs::read_to_string("./tests/data/main.rs").unwrap();
        println!("{}",contents);
        let job:PostJob=PostJob { 
            source_code:contents, 
            language: "Rust".to_string(),
            user_id: 1, 
            contest_id: 1, 
            problem_id: 1 
        };
        let config_path: String = "./config.json".to_string();

        let config: config::Config =
            config::parse_from_file(config_path).expect("Config file format error.");

        compile(job,Data::new(config) , 10);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn run_bin() {
        let id=10;
        let problem=1;
        let index=1;;
        let time_limit=10000;
        let mem_limit=1000000;
        let bin_path=format!("oj_runtime_dir/job_{}/job.exe", id);
        let out_path=format!("oj_runtime_dir/job_{}/", id);
        let input_path=format!("problems/{}",problem);
        let res=runner::run(index, &input_path, &bin_path, &out_path, time_limit, mem_limit);
        match res {
            Ok(time)=>{
                log::info!("{} problem time is {}",problem,time);
            }
            Err(result)=>{
                log::warn!("{} problem runtime error {}",problem,result);
            }
        }
    }

    use walkdir::*;
    use runner::util::dirGet;
    #[test]
    fn dir_get() {

        let problem=1;
        let input_path=format!("problems/{}",problem);
        let mut counter=0;

        let fs=dirGet(&input_path, ".in");
        println!("{:?}",fs);
    }
    
    #[test]
    fn file_diff(){
        use runner::diff::*;

        let problem=1;
        let input_path=format!("problems/{}",problem);
        let out_path=format!("oj_runtime_dir/job_{}",14);
    
        println!("{}",diff_standard("problems/1/1.out","oj_runtime_dir/job_14/1.out"));
        println!("{}",diff_strict("problems/1/1.out","oj_runtime_dir/job_14/1.out"));
        
        println!("{}",runner::score_single(1, &input_path, &out_path))
    }
}




#[cfg(test)]
mod UserGetTest{
    use super::user::User;
    use super::user;

    #[test]
    fn GetUser() {
    //    user::getUser()
    }

}


#[cfg(test)]
#[allow(non_snake_case)]
mod submissionTest{
    use mysql::*;
    use mysql::prelude::*;
    use actix_web::web::Data;
    use crate::submission::{self, getById};
    use tokio::sync::Mutex;
    use actix_rt;
     #[actix_rt::test]
    async fn subInsert(){
        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap(); // 获取连接池
        let pool=Data::new(
            Mutex::new(pool.clone()));
        // Data::
        let sub=submission::createSubmission(
            &pool, 
            1, 1, 
        &"1203".to_string(),&"Rust".to_string() ,&"1xxxxxsdsadfa\"23".to_string()).await;

        match sub {
            Some(s)=>println!("{:?}",s),
            _=>print!("none\n"),
        }
    }

    #[actix_rt::test]
    async fn get(){
        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap();
        let pool=Data::new(
            Mutex::new(pool.clone()));
        let sub= getById(&pool,1);
        // println!("{:?}",sub.await.unwrap());
        match sub.await {
            Some(s)=>println!("{:?}",s),
            _=>print!("none\n"),
        }
    }

    #[actix_rt::test]
    async fn updateTest(){
        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap();
        let pool =Data::new(
            Mutex::new(pool.clone()));
        submission::update(&pool, format!("result=0"), format!("id=3")).await;
    }


    
    #[actix_rt::test]
    async fn compile_error_check(){
        use mysql::*;
        use mysql::prelude::*;
        use actix_web::web::Data;
        use crate::submission::{self, getById};
        use tokio::sync::Mutex;
        use actix_rt;
        use crate::handler;
        use crate::runner;
        use crate::config;
        use crate::submission::RESULTS;
        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap(); // 获取连接池
        let pool=Data::new(
            Mutex::new(pool.clone()));

        let config_path: String = "./config.json".to_string();

        let config: config::Config =
            config::parse_from_file(config_path).expect("Config file format error.");

        let config=Data::new(config);


        let body=handler::submissionHandler::SubmissionData{
            problem_id:1,
            contest_id:122,
            code:"xxxx".to_string(),
            language:"Rust".to_string(),
        };
        runner::judge(&pool, config, body).await;
        
        let sub=submission::getLatest(&pool).await;
        let result=sub.unwrap().result;
        assert!(result==RESULTS::COMPILE_ERROR,"result = {}",result);
    }

    #[actix_rt::test]
    async fn wrong_answer_check(){
        use mysql::*;
        use mysql::prelude::*;
        use actix_web::web::Data;
        use crate::submission::{self, getById};
        use tokio::sync::Mutex;
        use actix_rt;
        use crate::handler;
        use crate::runner;
        use crate::config;
        use crate::submission::RESULTS;
        use std::fs;

        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap(); // 获取连接池
        let pool=Data::new(
            Mutex::new(pool.clone()));

        let config_path: String = "./config.json".to_string();

        let config: config::Config =
            config::parse_from_file(config_path).expect("Config file format error.");

        let config=Data::new(config);
        let code=fs::read_to_string("./tests/data/main.rs").unwrap();

        let body=handler::submissionHandler::SubmissionData{
            problem_id:1,
            contest_id:122,
            code:code,
            language:"Rust".to_string(),
        };
        runner::judge(&pool, config, body).await;
        
        let sub=submission::getLatest(&pool).await;
        let result=sub.unwrap().result;
        assert!(result==RESULTS::WRONG_ANSWER,"result = {}",result);
        
    }

    #[actix_rt::test]
    async fn Accepted_check(){
        use mysql::*;
        use mysql::prelude::*;
        use actix_web::web::Data;
        use crate::submission::{self, getById};
        use tokio::sync::Mutex;
        use actix_rt;
        use crate::handler;
        use crate::runner;
        use crate::config;
        use crate::submission::RESULTS;
        use std::fs;

        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap(); // 获取连接池
        let pool=Data::new(
            Mutex::new(pool.clone()));

        let config_path: String = "./config.json".to_string();

        let config: config::Config =
            config::parse_from_file(config_path).expect("Config file format error.");

        let config=Data::new(config);
        let code=fs::read_to_string("./tests/data/main2.cpp").unwrap();

        let body=handler::submissionHandler::SubmissionData{
            problem_id:100,
            contest_id:122,
            code:code,
            language:"C++".to_string(),
        };
        runner::judge(&pool, config, body).await;
        
        let sub=submission::getLatest(&pool).await;
        let result=sub.unwrap().result;
        assert!(result==RESULTS::ACCEPTED,"result = {}",result);

    }

}