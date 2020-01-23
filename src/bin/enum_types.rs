enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direct:Direction = Direction::Right; // Right....

    match player_direct { // works like switch statement
        Direction::Up => println!("Going Up."),
        Direction::Down => println!("Going Down."),
        Direction::Left => println!("Going Left."),
        Direction::Right => println!("Going Right.") // ||
    }
}
