/* Error Handling
 */

// Using closures (the unwrap_or_else method):
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

// Propagating Errors: Return the error to the calling code
// use std::fs::File;
// use std::io::{self, Read};

// fn main() {
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let username_file_result = File::open("hello.txt");

//         let mut username_file = match username_file_result {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         };

//         // Read contents of the file into username
//         let mut username = String::new();
//         match username_file.read_to_string(&mut username) {
//             Ok(_) => Ok(username),
//             Err(e) => Err(e),
//         }
//     }
//     read_username_from_file(); // This will print the error if it occurs
// }
