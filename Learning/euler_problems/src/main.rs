fn main() {
    multiples_of_3_and_5();
    even_fibonacci_numbers();
}

// Multiples of 3 and 5
fn multiples_of_3_and_5() {
    let mut sum = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
            println!("{}", i); // print each number that is divisible by either 3 or 5
        }
    }

    println!("The sum of all multiples of 3 or 5 below 1000 is: {}", sum);
}

// Even Fibonacci numbers
fn even_fibonacci_numbers() {
    let mut sum = 0;
    let (mut a, mut b) = (1, 2);

    while b <= 1_000 {
        if b % 2 == 0 {
            sum += b;
        }
        let temp = a + b;
        println!("{}", temp); // print each even Fibonacci number
        a = b;
        b = temp;
    }

    println!(
        "The sum of even Fibonacci numbers below one thousand is: {}",
        sum
    );
}
