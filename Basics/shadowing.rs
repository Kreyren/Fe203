fn main() {
    let mut x = 10;
    // shadowing...
    {
        let x = 20; // will be ignored
    }

    let x = 11;
    let x = false;
    println!("{}", x);
}
