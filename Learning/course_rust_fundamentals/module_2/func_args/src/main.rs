/* Practice with function arguments
Allow user to enter size and elements of a vector.
Calculate and print the sum and average of the vector.
*/

use std::io::{stdin, Read};

fn sum_and_average(numbers: &[i32]) -> (i32, f32) {
    let mut sum = 0;
    let count = numbers.len();

    for number in numbers {
        sum += number;
    }

    let average = sum as f32 / count as f32;
    (sum, average)
}

fn main() {
    let mut input = String::new();
    println!("Enter the number of elements:");
    stdin().read_line(&mut input).expect("Failed to read line");
    let num_elements: usize = input.trim().parse().expect("Invalid number of elements");

    let mut numbers: Vec<i32> = Vec::new();
    for _ in 0..num_elements {
        println!("Enter a number:");
        input.clear();
        stdin().read_line(&mut input).expect("Failed to read line");
        let number: i32 = input.trim().parse().expect("Invalid number");
        numbers.push(number);
    }

    let (sum, average) = sum_and_average(&numbers);
    println!("The sum is {}", sum);
    println!("The average is {}", average);
}
