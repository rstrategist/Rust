fn main() {
    println!("Now running the loop with break function:");
    loop_with_break();
    println!("Now running the labelled loop within a loop function:");
    loop_loop();
    println!("Now running the conditional loop with while function:");
    loop_while();
    println!("Now running the loop through collection function:");
    loop_collection();
    println!("Now running the concise loop through collection function:");
    loop_collection_concise();
    println!("Now running the for range function:");
    for_range();
}

fn loop_with_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn loop_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn loop_while() {
    let mut number = 50;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}

fn loop_collection_concise() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
