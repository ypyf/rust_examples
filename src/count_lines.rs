use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut files = Vec::new();
    for arg in std::env::args().skip(1) {
        files.push(arg);
    }
    for file in &files {
        match count_lines(file) {
            Err(e) => println!("{}", e),
            Ok(count) => println!("文件 '{}' 有{}行", file, count),
        }
    }
}

fn count_lines(file_name: &str) -> Result<usize, io::Error> {
    let mut f = File::open(file_name).expect("打开文件失败");
    let mut s = String::new();
    let mut count = 0;
    try!(f.read_to_string(&mut s));
    for c in s.chars() {
        if c == '\n' {
            count += 1;
        }
    }
    Ok(count)
}
