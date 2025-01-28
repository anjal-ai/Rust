enum Shape {
    Rectangle(f64, f64), // Rectangle(width, height)
    Circle(f64), //Circle(radius)
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    calculate_area(rect);

    let circle = Shape::Circle(3.0);
    calculate_area(circle);
}

fn calculate_area(shape : Shape){
    match shape {
        Shape::Rectangle(width, height) => {
            println!("Area of rectangle: {}", width * height);
        },
        Shape::Circle(radius) => {
            println!("Area of circle: {}", 3.14 * radius * radius);
        }
    }
}