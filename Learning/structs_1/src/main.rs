use std::io;

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Implementation block for Rectangle to define a function
// for calculating the area.
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let scale = 2.0;

    println!("Enter the height (in meters):");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");
    let height: f64 = height.trim().parse().expect("Please enter a valid number");

    println!("Enter the width (in meters):");
    let mut width = String::new();
    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line");
    let width: f64 = width.trim().parse().expect("Please enter a valid number");

    let rect1 = Rectangle {
        width: dbg!(width * scale), // Debug macro used here for demonstration purposes only!
        height,
    };

    println!("The area is {} square meters.", rect1.area());
    println!("rect1 is a {:#?}", rect1);
    dbg!(&rect1);
}
