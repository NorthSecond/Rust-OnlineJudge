


#[cfg(test)]
mod base64Test{
    use base64::encode;
    use base64::decode;
    use std::str;
    #[test]
    fn encodeTest(){
        let contents=std::fs::read_to_string("./tests/data/main.rs").unwrap();
        let encoded = encode(contents);
        println!("Base64: {}", encoded);

        let decoded =&decode(encoded).unwrap()[..];
        println!("decode {}",str::from_utf8(decoded).unwrap())
    }

}