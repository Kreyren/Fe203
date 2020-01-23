struct Color{
    red:u8,
    green:u8,
    blue:u8
}

fn main() {
    let blue = Color { red: 0, green: 0, blue: 225 };
    print_color(&blue);
}

fn print_color(colour: &Color) {
    println!("R:{}, G:{}, B:{}", colour.red, colour.green, colour.blue);
}
