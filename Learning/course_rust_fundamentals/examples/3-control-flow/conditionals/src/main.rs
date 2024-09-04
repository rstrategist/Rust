fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    let age = 27;
    let gender = "Male";

    // Nested if-else statements
    if age < 13 {
        println!("Child");
    } else if age < 20 {
        println!("Teenager");
        if gender == "Male" {
            println!("Teenage boy");
        } else {
            println!("Teenage girl");
        }
    } else {
        if gender == "Male" {
            println!("Adult male");
        } else {
            println!("Adult female");
        }
    }

    // Height
    // if height > 180 {
    //     println!("Tall");
    // } else if height > 160 {
    //     println!("Average");
    // } else {
    //     println!("Short");
    // }

    // // Age
    // if age < 13 {
    //     println!("Child");
    // } else if age < 20 {
    //     println!("Teenager");
    // } else {
    //     println!("Adult");
    // }

    // Using match instead
    // match age {
    //     0..=12 => println!("Child"),
    //     13..=19 => println!("Teenager"),
    //     _ => println!("Adult"),
    // }
}
