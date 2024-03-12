fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("The number {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number {number} is divisible by 2");
    } else {
        println!("The number {number} is not divisible by 4, 3, or 2");
    }
    assignment_with_if()
}

// Both variables in an if condition must be of the same type
fn assignment_with_if() {
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number}");
}