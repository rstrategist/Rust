/* This Rust program demonstrates how to modularise file I/O operations using 
separate functions for reading from and writing to a file. The program 
also incorporates error handling using the `match` construct to ensure 
that both reading and writing are handled safely and explicitly.

Key Concepts and Learnings:
1. **File Reading and Writing**: The program reads from and writes to files 
   using `File::open` and `File::create` methods, respectively.
2. **Error Handling**: Errors that may occur during file operations 
   (e.g., file not found, permission denied) are handled using the `match` 
   construct. This approach ensures that any potential errors are managed 
   gracefully rather than allowing the program to panic unexpectedly.
3. **Separation of Concerns**: By defining `read_from_file` and 
   `write_to_file` as separate functions, the code becomes more modular and 
   easier to maintain. Each function handles a specific task and can be 
   reused as needed.
4. **Default Behavior on Failure**: If the file to be read does not exist, 
   the program demonstrates how to perform an alternative action (such as 
   writing to a new file) rather than simply exiting or panicking.
5. **BufReader for Efficient Reading**: The program uses `BufReader` to 
   efficiently read lines from a file, which is useful for large files where 
   reading the entire file into memory at once may not be practical.
*/

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    // Attempt to read from a file
    read_from_file("non_existent_file.txt");

    // Attempt to write to a file
    write_to_file("output.txt", "Writing some text to the file.");
}

/// Function to read lines from a file
fn read_from_file(filename: &str) {
    let file = File::open(filename);

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found: {}. Attempting to write to file...", error);
                    write_to_file("output.txt", "Hello, Rust!");  // Write to the file if reading fails
                    return;  // Exit the function after writing
                }
                _ => {
                    panic!("Error opening file: {}", error);
                }
            }
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error);
            }
        }
    }
}

/// Function to write content to a file
fn write_to_file(filename: &str, content: &str) {
    let mut file = File::create(filename);

    match file {
        Ok(ref mut file) => {
            let result = file.write_all(content.as_bytes());
            match result {
                Ok(_) => println!("Successfully wrote to the file: {}", filename),
                Err(error) => println!("Error writing to file: {}", error),
            }
        }
        Err(error) => {
            match error.kind() {
                io::ErrorKind::PermissionDenied => {
                    println!("Permission denied: {}", error);
                }
                _ => {
                    println!("Error creating file: {}", error);
                }
            }
        }
    }
}
