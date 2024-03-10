use ordered_hash_map::OrderedHashMap;
use rand::Rng;
use std::{fs::OpenOptions, io::Write, str::from_utf8};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]).expect("can`t read file");
    let mut bytes = Vec::<[u8; 10]>::new();
    for (i, _) in content.char_indices() {
        let mut buf = Vec::<u8>::new();
        if i % 10 == 0 {
            content.chars().enumerate().for_each(|(ic, cc)| {
                if ic >= i - 10 && ic <= i - 1 {
                    buf.push(cc as u8)
                }
            })
        } else if content.len() - i + 1 < 10 {
            content.chars().enumerate().for_each(|(ic, cc)| {
                if ic >= content.len() - ((content.len() - 1) % 10) - 1 {
                    buf.push(cc as u8)
                }
            })
        }
        if buf.len() >= 1 {
            while buf.len() < 10 {
                buf.push(
                    from_utf8(&[0])
                        .expect("can`t write byte 0")
                        .as_bytes()
                        .get(0)
                        .expect("can`t get byte 0")
                        .to_owned(),
                );
            }
            bytes.push(buf.try_into().expect("can`t push bytes"))
        }
    }
    let mut rng = rand::thread_rng();
    let mut codes: OrderedHashMap<[u8; 10], [u8; 10]> = OrderedHashMap::new();
    let mut vec = vec![];
    for i in 0..128u8 {
        vec.push(i)
    }
    let alphabet: [u8; 128] = vec.try_into().expect("can`t make alphabet");
    for byte in bytes.to_owned() {
        let code: [u8; 10] = [
            alphabet[rng.gen_range(0..128usize)],
            alphabet[rng.gen_range(0..128usize)],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        if !codes.contains_key(&byte) {
            let _ = codes.insert(byte, code);
        }
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
                let _ = std::mem::replace(bytes.get_mut(i).expect("can`t get mut bytes"), value);
            }
        }
    }
    for byte in bytes.iter() {
        file.write_all(&byte[..2]).expect("can`t write to file");
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
            .write(&format!("   {:?}{:?}\n", &key[..], &value[..2]).as_bytes())
            .expect("can`t write book data}");
    }
    book_file.write(b"}\n").expect("can`t write book end");
}
