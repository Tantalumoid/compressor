use std::{collections::HashMap, str::from_utf8};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut content_file = std::fs::read_to_string(&args[1]).expect("can`t open file");
    let content_book = std::fs::read_to_string(&args[2]).expect("can`t open book");
    let mut codes: HashMap<&str, &str> = HashMap::new();
    for line in content_book.lines() {
        if line == "BOOK" || line == "{" || line == "}" {
            continue;
        }
        let line = &line[5..];
        let key = &line[..10];
        let value = &line[14..line.len() - 2];
        println!("{} --- {}", &value, &key);
        codes.insert(key, value);
    }
    println!("{:?}", codes);
    for (k, v) in codes {
        content_file = content_file.replace(v, k)
    }
    println!("{content_file}")
}
