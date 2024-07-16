use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let mut file = BufWriter::new(File::create("assets/levels/test.dat").unwrap());

    for _x in 0..60 {
        for _y in 0..60 {
            file.write_all(&[4]).unwrap();
            file.write_all(&01_u64.to_be_bytes()).unwrap();
        }
    }
}
