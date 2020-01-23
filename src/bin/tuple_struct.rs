struct Color(u8, u8, u8);

fn main() {
    let mut red = Color(69, 77, 00);
    println!("{}, {}, {}", red.0, red.1, red.2);
}
