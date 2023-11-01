use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("../input").unwrap());

    // let mut s = String::new();
    // f.read_line(&mut s).unwrap();

    let arr: Vec<Vec<&str>> = f.lines()
        .map(|l| l.unwrap().split(char::is_whitespace)
             .collect())
        .collect();

    println!("{:?}", arr);
}
