use actix_web::web::{self, route, Data};
use actix_web::{middleware::Logger, App, HttpServer};

use actix_web::{get, post, HttpResponse, Responder};

use mysql::*;
use mysql::prelude::*;
use tokio::sync::Mutex;

use clap::Arg;


mod user;
mod runner;
mod config;
mod error_log;
mod job;
mod handler;
mod submission;

//状态机
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //启动命令行客户端
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // let matches = clap::App::new("oj-backend")
    //     .version("0.0.1")
    //     .about("An online judge backend.")
    //     .arg(
    //         Arg::with_name("port")
    //             .short('p')
    //             .long("port")
    //             .takes_value(true)
    //             .help("The port of web."),
    //     )
    //     .arg(
    //         Arg::with_name("host")
    //             .short('h')
    //             .long("host")
    //             .takes_value(true)
    //             .help("The host of web."),
    //     )
    //     .get_matches();

    let mut address: String = "127.0.0.1".to_string();
    let mut port: u16 = 8000;
    let config_path: String = "./config.json".to_string();

    let config: config::Config =
        config::parse_from_file(config_path).expect("Config file format error.");
    let (address, port) = (
        config.server.bind_address.to_string(),
        config.server.bind_port,
    );

    // if(matches.is_present("host")){
    //     if let Some(arg) = matches.value_of("host") {
    //         address = String::from(arg);
    //     }
    //     println!("有host参数输入{}",address);
    // }
    // if(matches.is_present("port")){
    //     if let Some(arg) = matches.value_of("port") {
    //         port = arg.parse::<u16>().unwrap();
    //     }
    //     println!("有port参数输入{}",port);
    // }
    
    let mysql_url=format!("mysql://{}:{}@{}/{}",
            config.database.name,
            config.database.password,
            config.database.db_ip,
            config.database.db_name
        );
    log::info!("starting HTTP server at http://{}:{}", address, port); //config.server.bind_address, config.server.bind_port);
    log::info!("mysql database server at {}",mysql_url);

    let pool = Pool::new(&mysql_url[..]).unwrap(); // 获取连接池
    // let mut conn = pool.get_conn().unwrap();// 获取链接

    let mut conn = pool.get_conn().unwrap();// 获取链接

    conn.query_iter("select * from `tb_user`")
        .unwrap()
        .for_each(|row| {
            let r: (String, String, i32, String) = from_row(row.unwrap());
            log::info!("查询默认用户{}, {}, {}, {}", r.0, r.1, r.2, r.3);
        });

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(web::Data::new(
                config.clone()
            ))
            .app_data(web::Data::new(
                Mutex::new(pool.clone())
            ))
            .service(echo)
            .service(index)
            .configure(handler::route)
    })
    .bind((address, port))?
    .run()
    .await
}
