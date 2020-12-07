use std::io::stdin;
use std::io::stdout;
use std::io::{Read, Write};

struct FooReader {}

impl FooReader {
    fn read(&self, b: &mut String) -> Result<usize, std::io::Error> {
        print!("in> ");
        stdout().flush().unwrap();
        stdin().read_line(b)
    }
}

struct FooWriter {}

impl FooWriter {
    fn write(&self, b: &String) -> Result<usize, std::io::Error> {
        print!("out> ");
        stdout().write(b.as_bytes())
    }
}

fn main() {
    let reader = FooReader {};
    let writer = FooWriter {};

    let mut input = String::new();

    let s = match reader.read(&mut input) {
        Ok(s) => s,
        Err(_) => panic!("Unable to read data"),
    };
    println!("Read {} bytes from stdin", s);

    let s = match writer.write(&input) {
        Ok(s) => s,
        Err(_) => panic!("Unable to write data"),
    };
    println!("Write {} bytes from stdin", s);
}
