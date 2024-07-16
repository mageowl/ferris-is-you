use std::io::Write;
use std::{collections::HashMap, fs::File, io::BufWriter};

enum Object {
    Noun(u64),
    Is,
    Property(u64),
    Object(u64),
}

fn main() {
    let mut file = BufWriter::new(File::create("assets/levels/baba_00.dat").unwrap());

    let map = HashMap::from([
        //
        // TEXT //
        ((0, 0), Object::Noun(1)),
        ((1, 0), Object::Is),
        ((2, 0), Object::Property(1)),
        ((8, 0), Object::Noun(3)),
        ((9, 0), Object::Is),
        ((10, 0), Object::Property(4)),
        ((0, 8), Object::Noun(2)),
        ((1, 8), Object::Is),
        ((2, 8), Object::Property(3)),
        ((8, 8), Object::Noun(5)),
        ((9, 8), Object::Is),
        ((10, 8), Object::Property(2)),
        //
        // FERRIS //
        ((1, 4), Object::Object(1)),
        //
        // FLAG //
        ((9, 4), Object::Object(3)),
        //
        // ROCKS //
        ((5, 3), Object::Object(5)),
        ((5, 4), Object::Object(5)),
        ((5, 5), Object::Object(5)),
        //
        // WALLS //
        ((0, 2), Object::Object(2)),
        ((1, 2), Object::Object(2)),
        ((2, 2), Object::Object(2)),
        ((3, 2), Object::Object(2)),
        ((4, 2), Object::Object(2)),
        ((5, 2), Object::Object(2)),
        ((6, 2), Object::Object(2)),
        ((7, 2), Object::Object(2)),
        ((8, 2), Object::Object(2)),
        ((9, 2), Object::Object(2)),
        ((10, 2), Object::Object(2)),
        ((0, 6), Object::Object(2)),
        ((1, 6), Object::Object(2)),
        ((2, 6), Object::Object(2)),
        ((3, 6), Object::Object(2)),
        ((4, 6), Object::Object(2)),
        ((5, 6), Object::Object(2)),
        ((6, 6), Object::Object(2)),
        ((7, 6), Object::Object(2)),
        ((8, 6), Object::Object(2)),
        ((9, 6), Object::Object(2)),
        ((10, 6), Object::Object(2)),
        //
        // TILES //
        ((0, 3), Object::Object(4)),
        ((1, 3), Object::Object(4)),
        ((2, 3), Object::Object(4)),
        ((3, 3), Object::Object(4)),
        ((4, 3), Object::Object(4)),
        ((6, 3), Object::Object(4)),
        ((7, 3), Object::Object(4)),
        ((8, 3), Object::Object(4)),
        ((9, 3), Object::Object(4)),
        ((10, 3), Object::Object(4)),
        ((0, 4), Object::Object(4)),
        ((2, 4), Object::Object(4)),
        ((3, 4), Object::Object(4)),
        ((4, 4), Object::Object(4)),
        ((6, 4), Object::Object(4)),
        ((7, 4), Object::Object(4)),
        ((8, 4), Object::Object(4)),
        ((10, 4), Object::Object(4)),
        ((0, 5), Object::Object(4)),
        ((1, 5), Object::Object(4)),
        ((2, 5), Object::Object(4)),
        ((3, 5), Object::Object(4)),
        ((4, 5), Object::Object(4)),
        ((6, 5), Object::Object(4)),
        ((7, 5), Object::Object(4)),
        ((8, 5), Object::Object(4)),
        ((9, 5), Object::Object(4)),
        ((10, 5), Object::Object(4)),
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
