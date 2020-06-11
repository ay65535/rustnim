extern crate rand;

use std::io;
use std::path::Path;

fn main() {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Show`able structure
    // `display`メソッドは`Show`可能な構造体を返す。
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns the new path
    // `join`はOS固有のセパレータによってバイトのコンテナ型であるパス
    // を結合し、新しいパスを返す。
    let new_path = path.join("a").join("b");

    // Convert the path into a string slice
    // パスを文字列のスライスに変換する。
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

    println!("Please input SS bin path.");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    let path = Path::new(&buf);

    // Convert the path into a string slice
    // パスを文字列のスライスに変換する。
    match path.parent() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => match s.to_str() {
            None => panic!("convert failed"),
            Some(s) => println!("parent: {}", s)
        },
    }

    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    println!("file_stem: {}", file_stem);

    let extension = path.extension().unwrap().to_str().unwrap();
    println!("extension: {}", extension);
}
