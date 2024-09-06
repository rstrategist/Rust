/* CLI to open a file

Command-Line Argument Handling:
- The args vector collects command-line arguments.
- args[0] is the program name, and args[1] is the file path provided by the user.

Usage Message:
- Added a check to ensure a file path is provided. If not, it prints a usage message and exits with a non-zero status code.

Error Handling:
- Used eprintln! to print errors to standard error.
- Used std::process::exit(1) to exit the program with a non-zero status code in case of errors.

Run with: cargo run -- your_file_path.txt
*/

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user has provided a file path argument
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // The second argument is the file path provided by the user
    let file_path = &args[1];

    // Open the file at the provided path
    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("File not found: {}", error);
                    std::process::exit(1);
                }
                _ => {
                    eprintln!("Error opening file: {}", error);
                    std::process::exit(1);
                }
            }
        }
    };

    // Read and print lines from the file
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                std::process::exit(1);
            }
        }
    }
}
