struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let my_rectangle = Rectangle { width: 10, height: 6 };
    my_rectangle.print_description();
    println!("Rectangle is Square: {}", my_rectangle.is_square());
}
