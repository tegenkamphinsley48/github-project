use std::ops::Add;

fn main() {
    let numbers = vec![1, 2, 3];
    let sum = numbers.iter().sum();
    println!("The sum of {:?} is {}", numbers, sum);
}
