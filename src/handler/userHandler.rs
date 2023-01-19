
use actix_web::{delete, get, post, web,web::Data};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use super::super::config::Config;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct PostUser {
    pub user_name: String,
    pub user_password: String,
    pub sex: u8,
    pub email:String, 
}

#[get("/dbTest")]
async fn userlogin( 
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

