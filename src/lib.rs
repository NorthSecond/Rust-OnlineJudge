mod user;
mod runner;
mod config;
mod error_log;
mod job;
mod handler;
mod submission;
mod problem;


#[cfg(test)]
mod compileTest {
    use std::fs;
    use crate::job::PostJob;

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
    fn another() {
        
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
        submission::update(pool, format!("result=0"), format!("id=3")).await;
    }


}