use actix_web::http::header::ContentType;
use actix_web::{delete, get, post, web,web::Data};
use actix_web::{HttpResponse, Responder};
use mysql::binlog::jsonb::Array;
use serde::{Deserialize, Serialize};
use serde_json;
use super::super::config::Config;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Problems {
    problem_id: u64,
    problem_title: String,
    problem_path: String
}

#[get("/api/problem")]
async fn getProblems ( 
    // body: web::Json<PostJob>,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{

    let mut conn=pool.lock().await.get_conn().unwrap();// 获取链接
    let mut problem: Problems = Problems { 
        problem_id: 0, 
        problem_title: "test problem".to_string(), 
        problem_path: "test problem".to_string() 
    };

    conn.query_iter("select * from `tb_problem`")
        .unwrap()
        .for_each(|row| {
            let r:(u64, String, String) = from_row(row.unwrap());
            log::info!("查询题库{}, {}, {}", r.0, r.1, r.2);
            problem.problem_id=r.0;
            problem.problem_title=r.1;
            problem.problem_path=r.2;
    });

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&problem)
        .unwrap());

}
