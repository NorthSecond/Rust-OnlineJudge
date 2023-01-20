use actix_web::middleware::Condition;
use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;


#[derive(Clone,Default)]
pub struct User{
    pub username:String,
    pub password:String,
    pub sex:u8,
    pub email:String,
}


pub async fn getUsers(
    pool: web::Data<Mutex<Pool>>,
    condition:&String
) -> Result<Vec<User>> {
    let mut conn=pool.lock().await.get_conn().unwrap();
    let users=conn.query_map(
        format!("select * from tb_user {}",condition), 
        |(user_name,user_password,sex,email)|
        { 
            User{username:user_name,password:user_password,sex:sex,email:email}
        },
    );
    users
}

pub async fn getUserByName(
    pool: web::Data<Mutex<Pool>>,
    name:&String
)->Option<User>{
    let Rusers=
    getUsers(pool, &format!("where user_name='{}'",name)).await;

    match Rusers{
        Ok(users)=>users.get(0).cloned(),
        Err(info)=>None,
    }
}



pub async fn createUser(
    pool: web::Data<Mutex<Pool>>,
    name: &String,
    password: &String,
    email: &String
) -> Option<User> {
    println!("Create User...");
    println!("name:{}, password:{}, email:{}", name, password, email);
    let mut conn=pool.lock().await.get_conn().unwrap();
    // 检查username是否存在
    match getUserByName(pool, name).await {
        // 若username已存在则返回None
        Some(user) => {
            return None;
        }
        // username不存在则创建
        None => {
            conn.exec_drop("insert into `tb_user`(`user_name`, `user_password`, `email`) values (:n, :p, :e);", 
            params!{
                "n" => name, 
                "p" => password, 
                "e" => email,
            }).unwrap();
            let user = User {
                username:name.to_string(),
                password:password.to_string(),
                email:email.to_string(),
                sex:1,
            };
            Some(user)
        }
    }

}

