fn main() {
    // for loop
    for number in 0..101 { // 0 to 100
        println!("Loop:{}", number);
    }
    let num = 60..70;
    for (index, numbers) in num.enumerate() {
        println!("Loop {}:{}", index, numbers);
    }
}
