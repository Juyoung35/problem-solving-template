use std::io::{stdin, Read};
fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();

    let res = 0;

    println!("{res}");
}
