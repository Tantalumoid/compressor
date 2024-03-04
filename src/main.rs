use nanoid::nanoid;
use std::{collections::HashMap, fmt::format, io::Write};
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]).unwrap();
    let words: Vec<&str> = content.split_word_bounds().collect();
    let mut codes: HashMap<&str, String> = HashMap::new();
    let alphabet = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '`', ':', '~', '!', '@', '#', '$', '%', '^', '&',
        '*', '(', ')', '-', '_', '=', '+', ';', '/', '?', '.', ',', '<', '>',
    ];
    for word in words.iter() {
        *codes.entry(word).or_insert(nanoid![1, &alphabet]) += &nanoid![1, &alphabet];
    }
    println!("{:?}", codes);
    let mut vec_code_content = Vec::<String>::new();
    for word in words.iter() {
        codes.iter().for_each(|c| {
            if word != &&word.replace(*c.0, c.1) {
                vec_code_content.push(word.replace(*c.0, c.1))
            }
        });
    }
    let str_code_content = vec_code_content.into_iter().collect::<String>();
    println!("{str_code_content}");
    let file_path = format!(
        "{}_compressed.{}",
        &args[1].rsplit_once('.').unwrap().0,
        &args[1].rsplit_once('.').unwrap().1
    );
    println!("{file_path}");
    std::fs::File::create(file_path)
        .unwrap()
        .write(str_code_content.as_bytes());
}
