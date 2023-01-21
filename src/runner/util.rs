
use walkdir::WalkDir;
pub fn dirGet(path:&String,end:&'static str)->Vec<String> {

    // let problem=1;
    // let input_path=format!("problems/{}",problem);
    let mut counter=0;
    let mut f_names:Vec<String>=Vec::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
            let f_name = String::from(entry.file_name().to_string_lossy());
            if f_name.ends_with(end) {
                counter += 1;
                // println!("{}", f_name);
                f_names.push(f_name);
            }   
    };
    return f_names;
}   