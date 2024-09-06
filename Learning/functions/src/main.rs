// Declare Car struct to describe vehicle with four named fields
#[derive(Debug)] // Derive Debug trait for Car
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) {

    // Use the values of the input arguments
    let car = Car {
        color,
        transmission,
        convertible,
        mileage: 0,  // All new cars start with zero mileage
    };

    // Print the car details for testing purposes
    println!("Car created: {:?}", car);
}

fn main() {
    // Example usage of car_factory
    let car1 = car_factory(String::from("Red"), Transmission::Manual, true);
    let car2 = car_factory(String::from("Blue"), Transmission::Automatic, false);
}
