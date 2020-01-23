fn main() {
    let mut x = 10;
    // shadowing...
    {
        let dom = &mut x;
        *dom += 1; // also need * to do this
    }

    println!("x = {}", x);
}
