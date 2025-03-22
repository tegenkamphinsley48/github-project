use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    println!("{}", data);
}
