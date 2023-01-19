use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use std::sync::Arc;
use crate::error_log;
use tokio::sync::Mutex;

//用于表示请求传来的Json对象
#[derive(Deserialize)]
struct LoginInfo {
   username: String,
   password: String,
}

pub async fn username_exists(pool: Data<Mutex<Pool<SqliteConnectionManager>>>, user_name: String) -> bool {
    let data = pool.lock().await.get().unwrap();
    let mut stmt;
    match data.prepare("SELECT * FROM `tb_user` WHERE `user_name` = :name;") {
        Ok(s) => stmt = s,
        _ => {
            return true;
        }
    };
    stmt.exists(&[(":name", user_name)]).unwrap()
}

pub async fn get_user(pool: Data<Mutex<Pool<SqliteConnectionManager>>>, user_name: String) -> Result<LoginInfo, HttpResponse> {
    let data = pool.lock().await.get().unwrap();
    let mut stmt;
    match data.prepare("SELECT * FROM `tb_user` WHERE `user_name` = :name;") {
        Ok(s) => stmt = s,
        _ => {
            return Err(error_log::EXTERNAL::webmsg("Database Error."));
        }
    }
    if !stmt.exists(&[(":name", user_name)]).unwrap() {
        return Err(error_log::NOT_FOUND::webmsg(&format!(
            "User {} not found.", user_name
        )));
    }
    let iter = stmt.query_map(&[(":name", user_name)], |row| {
        Ok(LoginInfo {
            username: row.get(0)?,
            password: row.get(1)?,
        })
    });
    match iter {
        Ok(mut ans) => Ok(ans.next().unwrap().expect("Unknown Error.")),
        _ => Err(error_log::EXTERNAL::webmsg("Database Error.")),
    }
}

//暂未获取邮箱等信息
pub async fn create_user(
    pool: Data<Mutex<Pool<SqliteConnectionManager>>>,user_name: String, user_password: String) -> Result<LoginInfo, HttpResponse> {
    println!("create user");
    // Check if user name exists before increase the counter.
    if username_exists(pool.clone(), user_name).await {
        return Err(error_log::INVALID_ARGUMENT::webmsg(&format!(
            "User name '{}' already exists.",
            user_name
        )));
    } else {
        println!("user name: {}", user_name);
        println!("user password: {}", user_password);
        let data = pool.lock().await.get().unwrap();
        if let Err(_) = data.execute(
            "INSERT INTO users (id, name) VALUES (?1, ?2);",
            params![user_id, user_name],
        ) {
            return Err(error_log::EXTERNAL::webmsg("Database Error."));
        }
    }

    Ok(LoginInfo {
        username: user_name,
        password: user_password,
    })
}

// #[post("/login")] //声明请求方式和请求路径，接受post方式请求/login路径
// async fn index(login_info: web::Json<LoginInfo>) -> impl Responder {
//     format!("Hello {}! password:{}",login_info.username , login_info.password)
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
// 　//启动http服务器
//     HttpServer::new(|| App::new().service(index))
//         .bind("127.0.0.1:8088")?
//         .run()
//         .await
// }
