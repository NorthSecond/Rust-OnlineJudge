mod user;
mod runner;
mod config;
mod error_log;
mod job;
mod handler;



#[cfg(test)]
mod compileTest {
    use std::fs;
    use crate::job::PostJob;

    // use std::io::BufReader;
    use super::runner::compile;
    use super::job;
    use super::config;
    use actix_web::web::Data;
    #[test]
    fn readfile() {

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
        panic!("Make this test fail");
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
