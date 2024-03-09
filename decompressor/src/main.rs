use std::{collections::HashMap, str::from_utf8};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut content_file = std::fs::read_to_string(&args[1]).expect("can`t open file");
    let content_book = std::fs::read_to_string(&args[2]).expect("can`t open book");
    let mut codes: HashMap<String, String> = HashMap::new();
    for line in content_book.lines() {
        //BOOK
        //{
        //   [43, 104, 101, 108, 108, 111, 13, 10, 32, 32]>/[126, 104, 1, 1, 1, 1, 1, 1, 1, 1]/
        //   [104, 101, 108, 108, 111, 32, 119, 111, 114, 108]>/[32, 76, 1, 1, 1, 1, 1, 1, 1, 1]/
        //   [100, 33, 45, 45, 121, 101, 115, 115, 115, 45]>/[61, 36, 1, 1, 1, 1, 1, 1, 1, 1]/
        //   [33, 33, 33, 33, 33, 33, 33, 33, 33, 33]>/[68, 58, 1, 1, 1, 1, 1, 1, 1, 1]/
        //}
        if line == "BOOK" || line == "{" || line == "}" {
            continue;
        }
        let mut key = String::new();
        let mut value = String::new();
        for (i, char) in line.chars().enumerate() {
            if char == ']' {
                key = line[4..i].to_string();
                value = line[i + 2..].to_string().replace("]", "");
                break;
            }
        }
        codes.insert(key, value);
    }
    dbg!(codes);
}
