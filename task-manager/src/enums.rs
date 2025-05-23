#[derive(Debug, Clone)]
pub enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

pub enum Location {
    Unknown,
    Anonymous,
    Known(i64, i64),
}

impl Location {
    pub fn display(&self) {
        match self {
            Location::Unknown => println!("Location is unknown"),
            Location::Anonymous => println!("Location is anonymous"),
            Location::Known(x, y) => println!("Location: ({}, {})", x, y),
        }
    }
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(base, height, _) => 0.5 * base * height,
        }
    }

    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Shape::Square(side) => 4.0 * side,
            Shape::Rectangle(width, height) => 2.0 * (width + height),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}
