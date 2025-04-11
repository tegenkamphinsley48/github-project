fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<isize> = input.split_whitespace().map(|s| s.parse::<isize>().unwrap()).collect();

    for &number in numbers.iter() {
        if number % 2 == 0 {
            println!("{}", number);
        }
    }
}
