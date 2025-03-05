fn main() {
    let rand = rand::thread_rng();
    let num1 = rand.gen_range(1..=10);
    let num2 = rand.gen_range(1..=10);
    println!("{} + {} = {}", num1, num2, num1 + num2);
}
