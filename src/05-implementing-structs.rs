struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // Static debug method that takes explicit width and height as parameters
    fn debug(width: u32, height: u32) -> String {
        format!("Rectangle {{ width: {}, height: {} }}", width, height)
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Debug: {}", Rectangle::debug(rect.width, rect.height));
}
