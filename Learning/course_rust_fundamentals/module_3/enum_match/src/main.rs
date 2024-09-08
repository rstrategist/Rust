/*  The program demonstrates the use of an enum, `FileSize`, to represent
different file sizes and formats them accordingly using the
`format_size` function.
*/
use std::env;

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

// Alternative way to define filesize, by implementing an enum function
impl FileSize {
    fn format_size(&self) -> String {
        match self {
            // dereference * and cast u64 value to f64 for the calculation
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", (*kb as f64) / 1000.0),
            FileSize::Megabytes(mb) => format!("{:.2} MB", (*mb as f64) / 1000.0),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", (*gb as f64) / 1000.0),
            FileSize::Terabytes(tb) => format!("{:.2} TB", (*tb as f64) / 1000.0),
        }
    }

    fn bytes(&self) -> u64 {
        match self {
            FileSize::Bytes(bytes) => *bytes,
            FileSize::Kilobytes(kb) => kb * 1000,
            FileSize::Megabytes(mb) => mb * 1000 * 1000,
            FileSize::Gigabytes(gb) => gb * 1000 * 1000 * 1000,
            FileSize::Terabytes(tb) => tb * 1000 * 1000 * 1000 * 1000,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument is the size that was used to call the program (e.g. 1 gb). Must use quotes to
    // read this as a single argument
    println!("Size is: {} and {}.", args[1], args[2]);

    let size_str = &args[1]; // Get a reference to the String
    let size = size_str
        .parse::<u64>() // Call parse() to convert to u64
        .unwrap_or_else(|_| {
            // Handle the error case
            eprintln!("Error: Invalid size value '{}'", size_str);
            std::process::exit(1); // Exit the program with a non-zero status
        });
    println!("Size is {:?}", size);

    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
        _ => FileSize::Terabytes(size / 1_000_000_000_000),
    };

    let mut formatted_size = filesize.format_size(size);
    println!("Formatted size is: {}", formatted_size);
}
//println!("Formatted size is: {}", formatted_size);

// let filesize = match formatted_size {
//     0..=999 => FileSize::Bytes(size),
//     1000..=999_999 => FileSize::Kilobytes(size / 1000),
//     1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
//     1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
//     _ => FileSize::Terabytes(size / 1_000_000_000_000),
// };

// println!("Filesize: {}", filesize.format_size());

// return Sizes showing all the different representations
// e.g. $ cargo run "24 mb"
// would return Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }

//let words: Vec<&str> = args.split_whitespace().collect();
//let words = args.split(' ').collect::<Vec<&str>>();
//println!("{:?}", words);

//let result = format_size(6870556088837399);
//println!("{}", result)
