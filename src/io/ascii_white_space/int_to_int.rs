use std::io::{stdin, Read};
fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<_int>().unwrap();

    let res = 0;

    println!("{res}");
}

use std::io::{self, prelude::*, BufWriter};
fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<_int>().unwrap();

    let reses = vec![];

    let mut o = BufWriter::new(io::stdout());
    for i in 0..reses.len() {
        writeln!(o, "{reses[i]}").unwrap();
    }
}