
use actix_web::web::{self, route, Data};
use actix_web::{middleware::Logger, App, HttpServer};

use actix_web::{get, post,HttpResponse, Responder};

use clap::Arg;

mod handler;


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
    let matches = clap::App::new("oj-backend")
    .version("0.0.1")
    .about("An online judge backend.")
    .arg(
        Arg::with_name("port")
            .short('p')
            .long("port")
            .takes_value(true)
            .help("The port of web."),
    )
    .arg(
        Arg::with_name("host")
            .short('h')
            .long("host")
            .takes_value(true)
            .help("The host of web."),
    )
    .get_matches();

    let mut address:String = "127.0.0.1".to_string();
    let mut port:u16=8000;


    if(matches.is_present("host")){
        if let Some(arg) = matches.value_of("host") {
            address = String::from(arg);
        } 
        println!("有host参数输入{}",address);
    }
    if(matches.is_present("port")){
        if let Some(arg) = matches.value_of("port") {
            port = arg.parse::<u16>().unwrap();
        } 
        println!("有port参数输入{}",port);
    }


    log::info!("starting HTTP server at http://{}:{}", address, port); //config.server.bind_address, config.server.bind_port);
    
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(echo)
            .service(index)
            .configure(handler::route)
    })
    .bind((address, port))?
    .run()
    .await
}
