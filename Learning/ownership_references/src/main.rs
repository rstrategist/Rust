fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    println!(
        "Variables passed with reference are not mutable by default.
        We can give permission to modify the passed variable reference:"
    );

    let mut s = String::from("hello"); // Delare the variable as a mutable one
    change(&mut s); // Pass the variable as a mutable referene to allow modification
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string)
}
