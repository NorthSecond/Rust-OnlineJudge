#[cfg(test)]
mod tests {
    use serde_json::{Value};

    #[test]
    fn jsonparser() {
            let json = r#"
        {
        "article": "how to work with json in Rust",
        "author": "tdep",
        "paragraph": [
            {
            "name": "untyped"
            },
            {
            "name": "strongly typed"
            },
            {
            "name": "writing json"
            }
        ]
        }
        "#;
            let parsed: Value = read_json(json);

            println!("\n\n The title of the article is {}", parsed["article"])
    }
    
    fn read_json(raw_json:&str) -> Value {
        let parsed: Value = serde_json::from_str(raw_json).unwrap();
        return parsed
    }
}