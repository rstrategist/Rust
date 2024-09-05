/* More examples of loops: for, while, loop
https://learn.microsoft.com/en-us/training/modules/rust-loop-expressions/3-for-while-loop
*/

fn main() {
let mut counter = 1;
// stop_loop is set when loop stops
let stop_loop = loop {
    counter *= 2;
    println!("Counter is: {}", counter);
    if counter > 100 {
        // Stop loop, return counter value
        break counter;
    }
};
// Loop should break when counter = 128
println!("Break the loop at counter = {}.", stop_loop);


// While expression
counter = 1;
while counter < 5 {
    println!("We loop a while...{}", counter);
    counter = counter + 1;
}


// For loop : we can iterate over any collection type, such as an array, vector, or hash map
let big_birds = ["ostrich", "peacock", "stork"];
for bird in big_birds {
    println!("Bird: {}", bird);
    }

for number in 0..=5 { // for number = 1 to 10, inclusive
    println!("{}", number * 2);
}
}
