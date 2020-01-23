fn main() {
    print_num_times(21);
}

fn print_num_times(num: u32) {
    for n in 1..num {
        if is_even(n) {
            println!("{} is even number.", n);
        } else {
            println!("{} is odd number.", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
