fn main() {
    println!("Hello, world!");
    // create a function to calulate the average price of several items
    let items: Vec<i32> = vec![1, 2, 3];
    let average_price = calculate_average_price(&items);
    println!("Average price: {}", average_price);
}
// function to calculate the average price of several items
fn calculate_average_price(items: &Vec<i32>) -> f64 {
    let mut total_price = 0;
    for item in items {
        total_price += item;
    }
    total_price as f64 / items.len() as f64
}
