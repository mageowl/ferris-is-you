use std::io::Write;
use std::{collections::HashMap, fs::File, io::BufWriter};

enum Object {
    Noun(u64),
    Is,
    Property(u64),
    Object(u64),
}

fn main() {
    let mut file = BufWriter::new(File::create("assets/levels/test.dat").unwrap());

    let map = HashMap::from([
        ((0, 0), Object::Noun(1)),
        ((1, 0), Object::Is),
        ((2, 0), Object::Property(1)),
        ((8, 0), Object::Noun(3)),
        ((9, 0), Object::Is),
        ((10, 0), Object::Property(4)),
        (),
    ]);

    for x in 0..30 {
        for y in 0..20 {
            if let Some(obj) = map.get(&(x, y)) {
                let bytes: Vec<u8> = match obj {
                    Object::Noun(id) => [&[1], &id.to_be_bytes()[..]].concat(),
                    Object::Is => vec![2, 0, 0, 0, 0, 0, 0, 0, 0],
                    Object::Property(id) => [&[3], &id.to_be_bytes()[..]].concat(),
                    Object::Object(id) => [&[4], &id.to_be_bytes()[..]].concat(),
                };

                file.write_all(&bytes).unwrap();
            } else {
                file.write_all(&[0]).unwrap();
            }
        }
    }
}
