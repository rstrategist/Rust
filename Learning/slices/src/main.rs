fn main() {
    let mut s = String::from("hello world");

    // Just playing with string slice pointers
    let hello = &s[..5]; // index 0 to 4, inclusive
    let world = &s[6..]; // index  6 to the end of the string
    println!("Slicing for the first 5 letters gives: {}", hello);
    println!(
        "Slicing from the 7th indx to the end of the string gives: {}",
        world
    );

    let word = first_word(&s); // word will get the value 5
    println!(
        "Selecting the first words by slicing at the first occurence of a space gives: {}",
        word
    );
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    println!("Let's now slice an array:");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    //println!("{}", result);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // first word by indexing to the point where a space was found
        }
    }

    &s[..]
}
