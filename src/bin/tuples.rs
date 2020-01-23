fn main() {
    let tupl = (69, "test", 69.69, false, (1, 2, 5));
    println!("{}", tupl.1);
    println!("{}", (tupl.4).1);
    let tupl2 = (69, "test", 69.69);
    let (a, b, c) = tupl2;
    println!("A is {}", a);
    println!("B is {}", b);
}
