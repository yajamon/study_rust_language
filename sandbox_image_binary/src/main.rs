use std::env;
use std::fs::File;
use std::io::Read;

mod bmp;
use bmp::BMP;

fn main() {
    let argument = env::args().skip(1).next().unwrap();
    println!("argument: {}", argument);

    let mut buf = Vec::new();
    let file = &mut File::open(&argument).unwrap();
    let _ = file.read_to_end(&mut buf);

    let bmp = BMP::new(&buf);
}
