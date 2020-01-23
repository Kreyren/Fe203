fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for nums in 0..numbers.len() {
        println!("{}", numbers[nums]);
    }
}

fn main2() {
    let numbers = [69; 3]; // num and how many times.

    for nums in 0..numbers.len() {
        println!("{}", numbers[nums]);
    }
}
