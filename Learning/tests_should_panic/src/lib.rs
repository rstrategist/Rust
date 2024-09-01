/* The test passes if the code inside the function panics;
the test fails if the code inside the function doesn’t panic.
 */

// Guess function should_panic if input not between 0 and 100
pub struct Guess {
    value: i32,
}

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.");
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic]
//     fn greater_than_100() {
//         Guess::new(200); // Causing the code to panic; a successful test
//     }
// }

// Alternative code for Guess where the new function panics
// with different messages depending on whether the value
// is too small or too large.

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    // Checks for this substring in the panic message
    fn greater_than_100() {
        Guess::new(200); // Change to e.g. -1 for error
    }
}
