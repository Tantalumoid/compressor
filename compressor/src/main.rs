use rand::Rng;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Seek, Write},
    os::windows::fs::MetadataExt,
};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]).expect("can`t read file");
    let mut bytes = Vec::<[u8; 10]>::new();
    for (i, _) in content.char_indices() {
        let mut buf = Vec::<u8>::new();
        if i % 10 == 0 {
            content.chars().enumerate().for_each(|(ic, cc)| {
                if i >= 10 && ic >= i - 10 && ic <= i - 1 {
                    buf.push(cc as u8)
                }
            })
        } else if i + 1 == content.len() - content.len() % 10 {
            content.chars().enumerate().for_each(|(ic, cc)| {
                if ic >= content.len() - ((content.len() - 1) % 10) - 1 {
                    buf.push(cc as u8)
                }
            })
        }
        if buf.len() >= 1 {
            while buf.len() < 10 {
                buf.push(b' ');
            }
            bytes.push(buf.try_into().expect("can`t push bytes"))
        }
    }
    let mut rng = rand::thread_rng();
    let mut codes: HashMap<[u8; 10], [u8; 10]> = HashMap::new();
    let mut vec = vec![];
    for i in 1..=128u8 {
        vec.push(i)
    }
    let alphabet: [u8; 128] = vec.try_into().expect("can`t make alphabet");
    for byte in bytes.to_owned() {
        let code: [u8; 10] = [
            alphabet[rng.gen_range(0..127usize)],
            alphabet[rng.gen_range(0..127usize)],
            alphabet[0],
            alphabet[0],
            alphabet[0],
            alphabet[0],
            alphabet[0],
            alphabet[0],
            alphabet[0],
            alphabet[0],
        ];
        codes.entry(byte).or_insert(code);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!(
            "{}_compressed.{}",
            &args[1].rsplit_once('.').expect("can`t split path 0").0,
            &args[1].rsplit_once('.').expect("can`t split path 1").1
        ))
        .expect("can`t create compresed file");
    for (i, byte) in bytes.clone().iter_mut().enumerate() {
        for (key, value) in codes.to_owned() {
            if byte.to_owned() == key {
                let _ = std::mem::replace(bytes.get_mut(i).unwrap(), value);
                file.rewind().expect("can`t rewind file");
                file.set_len(0).expect("can`t set len for file");
                file.write_all(&bytes[i]).expect("can`t write to file");
                println!(
                    "{:.3}kb",
                    file.metadata().unwrap().file_size() as f32 / 1024f32
                );
            }
        }
    }

    let mut book_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!(
            "{}_book.{}",
            &args[1].rsplit_once('.').expect("can`t split path 0").0,
            &args[1].rsplit_once('.').expect("can`t split path 1").1
        ))
        .expect("can`t create book of file");
    book_file.write(b"BOOK\n{\n").expect("can`t write book");
    for (key, value) in codes {
        book_file
            .write(
                &format!(
                    "\tKey-Start{:?}Key-End : Value-Start{:?}Value-End\n",
                    key, value
                )
                .as_bytes(),
            )
            .expect("can`t write book data}");
    }
    book_file.write(b"}\n").expect("can`t write book end");
}
