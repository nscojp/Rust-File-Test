use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    println!("{}!!", greet());

    let path = env::current_dir()?;
    let p_rsult = print_path(path);
    if p_rsult.is_err() {
        return Ok(());
    }
    //println!("starting dir: {}", path.display());

    Ok(())
}

fn greet() -> String {
    let greeting = format!("Hello, {}!", "Alice");
    println!("{}", greeting);
    greeting
}

fn print_path<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    // .as_ref() で &Path に変換して使う
    let p = path.as_ref();
    println!("パス: {}", p.display());

    for entry in fs::read_dir(p)? {
        let entry = entry?;
        let f_type = entry.path();
        let path = entry.file_name();
        if f_type.is_file() {
        } else {
            println!("{:?}", path);

            match junction::exists(path) {
                Ok(true) => println!("これはジャンクションです"),
                Ok(false) => println!("ジャンクションではありません"),
                Err(e) => println!("エラー: {}", e),
            }
        }
    }

    Ok(())
}
