
use actix_web::{delete, get, post, web,web::Data};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use super::super::{config::Config};
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;
use crate::user::*;



#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct PostUser {
    pub user_name: String,
    pub user_password: String,
    pub sex: u8,
    pub email:String, 
}


#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct LoginInfo{
    pub username:String,
    pub password:String,
}






#[post("/login")]
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
                HttpResponse::Ok().body("登录成功")
            }else{
                HttpResponse::Ok().body("密码错误")
            }
        },
        None=>{
            return HttpResponse::Ok().body("账号不存在");
        }
    }

}

#[get("/path")]
async fn extractor_multiple(p: web::Path<(String, String)>, q: web::Query<LoginInfo>) -> String {
    log::info!("p={:?}, q={:?}", p, q);

    return "dsadasd".to_string();
}

#[post("/POST")]
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



#[get("/dbTest")]
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

