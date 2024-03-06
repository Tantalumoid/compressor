use rand::Rng;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Seek, Write},
    os::windows::fs::MetadataExt,
};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]).unwrap();
    let mut words = Vec::<String>::new();
    for (i, _) in content.char_indices() {
        let mut str = String::new();
        if i % 2 == 0 && i % 10 == 0 {
            content.chars().enumerate().for_each(|(ic, cc)| {
                if ic >= i - 10 && ic <= i - 1 {
                    str = format!("{str}{cc}");
                }
                dbg!(cc);
            })
        }
        if str.len() >= 1 {
            words.push(str)
        }
    }
    let mut rng = rand::thread_rng();
    let mut codes: HashMap<&str, String> = HashMap::new();
    let mut vec = vec![];
    for i in 0..=127u8 {
        vec.push(i as char)
    }
    let alphabet: [char; 128] = vec.try_into().unwrap();
    for word in words.iter() {
        let mut str = String::new();
        str.push(alphabet[rng.gen_range(0..127usize)]);
        if word.len() > 5 {
            str.push(alphabet[rng.gen_range(0..127usize)]);
        }
        *codes.entry(word).or_insert(str) += "";
    }
    let mut vec_code_content = Vec::<String>::new();
    let mut str_code_content = String::new();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!(
            "{}_compressed.{}",
            &args[1].rsplit_once('.').unwrap().0,
            &args[1].rsplit_once('.').unwrap().1
        ))
        .unwrap();
    for word in words.iter() {
        codes.iter().for_each(|c| {
            if word != &word.replace(*c.0, c.1) {
                vec_code_content.push(word.replace(*c.0, c.1));
                str_code_content = vec_code_content.clone().into_iter().collect::<String>();
                file.rewind().unwrap();
                file.set_len(0).unwrap();
                file.write_all(str_code_content.as_bytes()).unwrap();
                println!(
                    "{:.3}kb",
                    file.metadata().unwrap().file_size() as f32 / 1024f32
                );
            }
        });
    }
    let mut book_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!(
            "{}_book.{}",
            &args[1].rsplit_once('.').unwrap().0,
            &args[1].rsplit_once('.').unwrap().1
        ))
        .unwrap();
    book_file
        .write(format!("BOOK\n{:#?}", codes).as_bytes())
        .unwrap();
}
