
use std::ptr::null;

use actix_web::http::header::ContentType;
use actix_web::{delete, get, post, web,web::Data};
use actix_web::{HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use serde_json;
use super::super::{config::Config};
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;

use crate::user::*;

use crate::error_log::LOGIN;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct PostUser {
    pub user_name: String,
    pub user_password: String,
    pub sex: u8,
    pub email:String, 
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub create_time: String,
    pub admin_type: String,
    pub problem_permission: String,
    pub reset_password_token: String,
    pub reset_password_token_expire_time: String,
    pub auth_token: String,
    pub two_factor_auth: bool,
    pub tfa_token: String,
    // pub sesseion_keys: String,
    pub open_api: bool,
    pub open_api_appkey: String,
    pub is_disable: bool,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct LoginInfo{
    pub username:String,
    pub password:String,
}

#[derive(Deserialize, Serialize)]
pub struct LOGIN_SUCCESS{
    pub data:String,
    pub result:bool,
}

#[post("/api/login")]
async fn userlogin(
    body: web::Json<LoginInfo>,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{
    // body.username

    // let users=user::getUsers(pool, format!("where user_name=");
    log::info!("用户登录 {:?}",body);
    match getUserByName(pool, &body.username).await{
        Some(user)=>{
            if(user.password==body.password){
                HttpResponse::Ok().json(
                    LOGIN_SUCCESS{
                        data:"login success".to_string(),
                        result:true
                    }
                )
            }else{
                // HttpResponse::Ok().body("密码错误")
                LOGIN::PASSWORD_ERROR("密码错误")
            }
        },
        None=>{
            return HttpResponse::Ok().body("账号不存在");
        }
    }
}

#[post("/api/tfa_required")]
async fn tfaRequiredCheck(
    // body: web::Json<LoginInfo>,
    body:String,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{
    log::info!("api/tfa_required {}",body);
    HttpResponse::Ok().json(
        LOGIN_SUCCESS{
            data:"login success".to_string(),
            result:false
        }
    )
}

<<<<<<< HEAD

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct RegisterInfo{
    pub username:String,
    pub password:String,
    pub email:String,
}


#[post("/register")]
async fn registerUser(
    body: web::Json<RegisterInfo>,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{
    
    log::info!("用户注册 {:?}",body);
   
    match createUser(pool, &body.username, &body.password, &body.email).await {
        Some(user) => {
            HttpResponse::Ok().body("注册成功")
        },
        None => {
            HttpResponse::Ok().body("注册失败")
        }
    }
}




#[get("/path")]
=======
#[get("/api/path")]
>>>>>>> 853b39a (update profile)
async fn extractor_multiple(p: web::Path<(String, String)>, q: web::Query<LoginInfo>) -> String {
    log::info!("p={:?}, q={:?}", p, q);

    return "dsadasd".to_string();
}

#[post("/api/POST")]
async fn postTest(
    body: String,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{
    // body.username
    // let users=user::getUsers(pool, format!("where user_name=");

    
    log::info!("Test post {:?}",body);
    let user:LoginInfo =serde_json::from_str(&body[..]).unwrap();
    log::info!("Test post user {:?}",user);
    return HttpResponse::NoContent().body(body);
}



#[get("/api/dbTest")]
async fn dbtest( 
    // body: web::Json<PostJob>,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
)-> impl Responder{
    let mut conn=pool.lock().await.get_conn().unwrap();// 获取链接
    let mut user:PostUser=PostUser { 
        user_name: "he".to_string(), 
        user_password: "he".to_string(), 
        sex: 0, 
        email: "dsad".to_string() 
    };
    conn.query_iter("select * from `tb_user`")
        .unwrap()
        .for_each(|row| {
            let r:(String, String, i32, String) = from_row(row.unwrap());
            log::info!("查询默认用户{}, {}, {}, {}", r.0, r.1, r.2, r.3);
            user.user_name=r.0;
            user.user_password=r.1;
    });
    return HttpResponse::Ok().body(serde_json::to_string_pretty(&user).unwrap());
}

#[get("/api/profile")]
async fn getUserInfo (
    body: web::Json<LoginInfo>,
    pool: Data<Mutex<Pool>>,
    config: Data<Config>
) -> impl Responder {
    log::info!("获取信息 {:?}",body);
    let mut user_info: User=User { 
        username: "default".to_string(), 
        email: "default".to_string(), 
        create_time: "2023-01-01".to_string(), 
        admin_type: "Regular User".to_string(), 
        problem_permission: "None".to_string(), 
        reset_password_token: "".to_string(), 
        reset_password_token_expire_time: "".to_string(), 
        auth_token: "".to_string(), 
        two_factor_auth: false, 
        tfa_token: "".to_string(), 
        open_api: false, 
        open_api_appkey: "".to_string(), 
        is_disable: false, 
    };
    match getUserByName(pool, &body.username).await {
        Some(user) => {
            user_info.username = user.username;
            user_info.email = user.email;
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string_pretty(&user_info).unwrap());
        }, 
        None => {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string_pretty(&user_info).unwrap());
        }
    }
}
