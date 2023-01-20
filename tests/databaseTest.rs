#[cfg(test)]
mod tests {
    use mysql::*;
    use mysql::prelude::*;
    use chrono::prelude::*; // 用来处理日期
    
    //查询数据
    #[test]
    fn query() {
        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap(); // 获取连接池
        let mut conn = pool.get_conn().unwrap();// 获取链接

        conn.query_iter("select * from `tb_user`")
            .unwrap()
            .for_each(|row| {
                let r: (String, String, i32, String) = from_row(row.unwrap());
                println!("{}, {}, {}, {}", r.0, r.1, r.2, r.3);
            });
    }

    //插入数据
    #[test]
    fn insert() {
        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap(); // 获取连接池
        let mut conn = pool.get_conn().unwrap();// 获取链接

        conn.exec_drop("Insert into `tb_user`(`user_name`, `user_password`, `email`) values (:name,:password,:email);", 
        params! {
            "name" => "test",
            "password" => "test",
            "email" => "test",
        }).unwrap();

        println!("Insert successfully!");
    }

    #[test]
    fn subInsert(){
        let url = "mysql://RUST-OJ:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap(); // 获取连接池
     
        let mut conn=pool.get_conn().unwrap();
    
        conn.exec_drop(
    "insert into `tb_submission`(`id`,`contest`, `problem`, `username`,`code`,`result`,`create_time`) values (null,:c, :p, :u,:code,:r,now());", 
        params!{
            "c" => 0, 
            "p" => 0, 
            "u" => "sss".to_string(),
            "code"=>"sss".to_string(),
            "r" => 6,
        }).unwrap();


        print!("测试通过");

    }

}