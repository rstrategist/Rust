/* Vector of enum Shapes (Square, Circle). Calculate total area by iterating
over the vector and matching appropriate formulae for each Shape enum variant
 */

#[derive(Debug, Clone)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

// Function takes a vector of shapes and returns the largest shape based on its area
fn largest_shape(shapes: &[Shape]) -> Shape {
    let mut largest = shapes[0].clone();
    for shape in shapes {
        if area(shape) > area(&largest) {
            largest = shape.clone();
        }
    }
    largest
}

// Function calculates the area of a given shape
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Triangle(side_a, side_b) => 0.5 * side_a * side_b,
    }
}

// Main function
fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle(10.0, 10.0),
    ];

    let largest = largest_shape(&shapes);
    println!("Largest shape: {:?}", largest);
    println!("Largest shape area: {:?}", area(&largest));
    println!("pi = {}", std::f64::consts::PI);
}
