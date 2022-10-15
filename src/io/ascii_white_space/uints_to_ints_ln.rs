use std::io::{self, prelude::*, BufWriter};
fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();

    let reses = vec![];

    let mut o = BufWriter::new(io::stdout());
    for i in 0..reses.len() {
        writeln!(o, "{reses[i]}").unwrap();
    }
}
