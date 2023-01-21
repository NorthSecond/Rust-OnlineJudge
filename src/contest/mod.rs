use actix_web::middleware::Condition;
use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;
use crate::handler::contestHandler::Contest;

pub async fn get_contests(
    pool: web::Data<Mutex<Pool>>,
) -> Result<Vec<Contest>> {
    let mut conn = pool.lock().await.get_conn().unwrap();
    let contests = conn.query_map(
        "select * from tb_contest", 
        |(   
            id,
            title,
            status,
            description,
            start_time,
            end_time,
            rule_type,
            created_by,
            passwork,
            real_time_rank ,
            visible  ,      
            last_update_time,
               
        )|
        { 
        
            log::info!("{}",last_update_time);
            
            Contest { 
                id: id,
                title: title, 
                description: description, 
                real_time_rank: real_time_rank, 
                passwork:passwork, 
                rule_type: rule_type, 
                start_time:start_time ,
                end_time: end_time, 
                last_update_time: last_update_time, 
                visible:visible, 
                status: status, 
                created_by:created_by 
            }
        },
    );
    contests
}