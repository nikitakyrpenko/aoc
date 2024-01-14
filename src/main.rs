mod first;

use crate::first::{convert, parse, read};

const PART_ONE_DATA: &str = "src/static/first/Sample.txt";

fn main() {
    let content: u32 = read(PART_ONE_DATA)
        .lines()
        .map(|line| parse(line))
        .fold(0, |acc, e| acc + convert(e));

    println!("{}", content)
}
