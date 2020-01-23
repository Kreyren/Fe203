fn main() {
    // here
    let x = 10;
    {
        let y = 20;
        // isolated
        println!("X:{} | Y:{}", x, y);
    }
}
