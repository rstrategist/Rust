// Vectors practice
fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn calculate_average(prices: &Vec<f64>) -> Option<f64> {
    // Sum all the prices in the vector
    let sum: f64 = prices.iter().sum();
    let length = prices.len() as f64;

    Some(sum / length)
}

fn append_vectors(v: &mut Vec<i32>, v2: &mut Vec<i32>) {
    v.extend(v2.iter().cloned()); // pass an iter over vector v2
}

fn main() {
    // let vec = vec![1, 2, 3, 4, 5];
    // get_item(3);

    // Retrieve a value at a specific index
    // let third_value = vec[2];
    // println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    // let last_value = vec.last().unwrap();
    // println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    // match vec.first() {
    //     Some(first_value) => println!("The first value in the vector is: {}", first_value),
    //     None => println!("The vector is empty!"),
    // }

    // calculate sum of a vector of prices
    // let mut prices = vec![10.0, 20.0, 30.0];
    // let average = calculate_average(&mut prices);

    // match average {
    //     Some(average) => println!("The average price is: {}", average),
    //     None => println!("The vector is empty!"),
    // }

    // Adding elements to a vector
    let mut v = vec![1, 2, 3];
    println!("{:?}", v); // Output: [1, 2, 3]
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]

    let mut v2 = vec![9, 10, 11, 12, 13, 14, 15, 16];
    println!("{:?}", v2);
    println!("{:?}", v);
    //v.append(&mut v2);
    //println!("{:?}", v);
    append_vectors(&mut v, &mut v2);
    println!("{:?}", v);
    // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
}
