use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content_file = std::fs::read_to_string(&args[1]).expect("can`t open file");
    let content_book = std::fs::read_to_string(&args[2]).expect("can`t open book");
    let mut codes: HashMap<&str, String> = HashMap::new();
    for line in content_book.lines() {
        if line == "BOOK" || line == "{" || line == "}" {
            continue;
        }
        let line = &line[5..];
        let key = &line[..10];

        let mut value = line[14..].to_string();
        value = value[..value.len() - 2].replace('"', "");
        codes.insert(key, value);
    }
    let mut decode_content_string = content_file.clone();
    for (k, v) in codes {
        if content_file.replace(&v, k).contains(k) {
            decode_content_string.push_str(&content_file.replace(&v, k));
        }
        println!("a a {content_file}")
    }
    println!(" b b{decode_content_string}")
}
