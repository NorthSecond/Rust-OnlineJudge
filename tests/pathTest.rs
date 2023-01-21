#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;
    #[test]
    fn test() {
        // 直接将一个字符串切片包装成一个路径切片
        let path = Path::new("./foo/bar.txt");

        // 返回上级路径，若无上级路径则返回 `None`
        let parent = path.parent().unwrap();

        // 返回文件名（不包含文件扩展名）
        let file_stem = path.file_stem().unwrap();

        println!(
            "path: {:?}, parent: {:?}, file_stem: {:?}",
            path, parent, file_stem
        );

        // 创建一个空的 `PathBuf`
        let mut empty_path = PathBuf::new();
        println!("empty_path: {:?}", empty_path);

        // 根据字符串切片创建 `PathBuf`
        let path = PathBuf::from(r"C:\windows\system32.dll");

        // 添加路径
        empty_path.push(r"C:\");

        println!("empty_path: {:?}, path: {:?}", empty_path, path);
    }
}