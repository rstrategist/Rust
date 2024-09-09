/* Function receives a Person struct and returns a String that contains the full name of the person.
Since the middle name is Option<String> it could have Some or None so needs to be appropriately handled
to avoid panic!
*/

struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    // Handle the person's middle name
    match &person.middle {
        Some(middle_name) => {
            full_name.push_str(&middle_name);
            full_name.push_str(" ");
        }
        None => {}
    }
    
    full_name.push_str(&person.last);
    full_name
}

fn main() {
    let rashid = Person {
        first: String::from("Rashid"),
        middle: Some(String::from("Mahmood")),
        last: String::from("Rasul"),
    };
    assert_eq!(build_full_name(&rashid), "Rashid Mahmood Rasul");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}
