use std::{fs::OpenOptions, io::Write, str::from_utf8};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content_file = std::fs::read_to_string(&args[1]).expect("can`t open file");
    let content_book = std::fs::read_to_string(&args[2]).expect("can`t open book");
    let mut decode_content = String::new();
    for line in content_book.lines() {
        if line == "BOOK" || line == "{" || line == "}" {
            continue;
        }
        let mut key = String::new();
        let mut value = String::new();
        for (i, char) in line.chars().enumerate() {
            if char == ']' {
                key = line[4..i].chars().rev().collect::<String>();
                key.push(' ');
                key = key
                    .chars()
                    .rev()
                    .collect::<String>()
                    .replace(", ", " ")
                    .replace(" 0", "");
                value = line[i + 2..].replace("]", "");
                break;
            }
        }
        value = value.replace(", ", " ");
        let mut bytes = Vec::<u8>::new();
        key.split_whitespace()
            .for_each(|str| bytes.push(str.parse().expect("can`t parse book")));
        let key = from_utf8(&bytes[..]).expect("can`t get utf8");
        let mut bytes = Vec::<u8>::new();
        value
            .split_whitespace()
            .for_each(|str| bytes.push(str.parse().expect("can`t parse book")));
        let value = from_utf8(&bytes[..]).expect("can`t get utf8");
        if content_file.contains(&value) {
            decode_content += &key;
        }
    }
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!(
            "{}_decode.{}",
            &args[1].rsplit_once('.').expect("can`t split path 0").0,
            &args[1].rsplit_once('.').expect("can`t split path 1").1
        ))
        .expect("can`t create decode of file")
        .write_all(decode_content.as_bytes())
        .expect("can`t write file");
}
