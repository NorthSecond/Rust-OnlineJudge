#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;
    use serde_json::{Value};
    use std::fs::File;
    use std::fs;
    #[test]
    fn test() {
        // println!("ok");

        let id = 1;
        let path = format!("./problems/1/problem.json");
        let path = Path::new(path.as_str());
        // let path = std::env::current_dir().unwrap();
        
        let problem_file = File::open(path).unwrap();
        //println!("ok");
        let content: serde_json::Value = serde_json::from_reader(problem_file).unwrap();
        
        //println!("{}", content["description"].to_string());
        // println!("{}", content["time_limit"]["value"].to_string().parse::<u64>().unwrap());
        println!("{}", content["memory_limit"]["value"].to_string().parse::<u64>().unwrap());

        // println!("{:?}", content["samples"][0]["input"]);

        struct Sample {
            input: String,
            output: String,
        }
        let mut sample:Sample = Sample{
            input: content["samples"][0]["input"].to_string(),
            output: content["samples"][0]["output"].to_string(),
        };

        println!("{}", sample.input);
        println!("{}", sample.output);

        // println!("{}", content["samples"]["input"].to_string());
        // println!("{}", content["samples"]["output"].to_string());
        println!("{}", content["hint"].to_string());
        println!("{}", content["source"].to_string());
    }

    #[test]
    fn current_path() {
        let paths = fs::read_dir("./problems/1").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    }
}