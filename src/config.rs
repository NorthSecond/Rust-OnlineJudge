use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

pub const RESULTS: &[&str] = &[
    "Waiting",
    "Running",
    "Accepted",
    "Compilation Error",
    "Compilation Success",
    "Wrong Answer",
    "Runtime Error",
    "Time Limit Exceeded",
    "Memory Limit Exceeded",
    "System Error",
    "SPJ Error",
    "Skipped",
];

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Ids {
    pub jobsid: u32,
    pub usersid: u32,
    pub contestsid: u32,
}

macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Deserialize, Serialize, Clone, Default, Debug)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!( Config {
    server: Server,
    database:Database,
    problems: Vec<Problem>,
    languages: Vec<Language>,
});

pub_struct!( Database{
    name:String,
    password:String,
    db_ip:String,
    db_name:String,
});


pub_struct!(Server {
    bind_address: String,
    bind_port: u16,
});

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Problem {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: ProbType,
    pub misc: Misc,
    pub cases: Vec<Case>,
}

pub_struct!( Misc {
    packing: Option<Vec<Vec<u32>>>,
    special_judge: Option<Vec<String>>,
});

pub_struct!(Case {
    score: f32,
    input_file: String,
    answer_file: String,
    time_limit: u64,
    memory_limit: u32,
});

pub_struct!( Language {
    name: String,
    file_name: String,
    command: Vec<String>,
});

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub enum ProbType {
    #[default]
    standard,
    strict,
    spj,
    dynamic_ranking,
}

pub fn parse_from_file(config_path: String) -> Result<Config, serde_json::Error> {
    let file = File::open(config_path).expect("Cannot read config file");
    serde_json::from_reader(BufReader::new(file)) //.expect("Config file has a wrong json format.")
}
