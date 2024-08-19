// Error Handling

// Handling different kinds of errors in different ways
// using closures, instead of nested match expressions
// utilising the unwrap_or_else method
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


// Handling different kinds of errors in different ways
// using nested match statements, which is not ideal.
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {other_error:?}");
//             }
//         },
//     };
// }


// Calling the panic! macro
// fn main() {
//     panic!("crash and burn");
// }

// Causing a panic by indexing outside of a vector
// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }


//  Using a match expression to handle the Result variants that might be returned
// use std::fs::File;
// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {error:?}"),
//     };
// }