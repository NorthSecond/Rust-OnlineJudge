CREATE USER 'RUST-OJ' @'%' IDENTIFIED WITH mysql_native_password BY '123456';
GRANT ALL PRIVILEGES ON *.* TO 'RUST-OJ' @'%';
FLUSH PRIVILEGES;
