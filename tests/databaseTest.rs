#[cfg(test)]
mod tests {
    use mysql::*;
    use mysql::prelude::*;
    use chrono::prelude::*; // 用来处理日期
    
    #[test]
    fn connect() {
        let url = "mysql://root:123456@localhost:3306/rustoj";
        let pool = Pool::new(url).unwrap(); // 获取连接池
        let mut conn = pool.get_conn().unwrap();// 获取链接


        conn.query_iter("select * from `tb_user`")
            .unwrap()
            .for_each(|row| {
                let r: (String, String, i32, String) = from_row(row.unwrap());
                println!("{}, {}, {}, {}", r.0, r.1, r.2, r.3);
            });
    }
}