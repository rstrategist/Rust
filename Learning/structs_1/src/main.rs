#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Implementation block for Rectangle to define associted
// functions for calculating the area and whether one rectangle
// can hold another.
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 1.0;

    // println!("Enter the height (in meters):");
    // let mut height = String::new();
    // io::stdin()
    //     .read_line(&mut height)
    //     .expect("Failed to read line");
    // let height: f64 = height.trim().parse().expect("Please enter a valid number");

    // println!("Enter the width (in meters):");
    // let mut width = String::new();
    // io::stdin()
    //     .read_line(&mut width)
    //     .expect("Failed to read line");
    // let width: f64 = width.trim().parse().expect("Please enter a valid number");

    // let rect1 = Rectangle {
    //     width: dbg!(width * scale), // Debug macro used here for demonstration purposes only!
    //     height: height,
    // };

    let rect1 = Rectangle {
        width: dbg!(30.0 * scale), // Debug macro used here for demonstration purposes only!
        height: 50.0,
    };
    let rect2 = Rectangle {
        width: 10.0,
        height: 40.0,
    };
    let rect3 = Rectangle {
        width: 60.0,
        height: 45.0,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The area is {} square meters.", rect1.area());
    println!("rect1 is a {:#?}", rect1);
    dbg!(&rect1);
}
