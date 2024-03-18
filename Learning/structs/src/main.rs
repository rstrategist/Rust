fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // Using struct update syntax. The syntax .. specifies
    // that the remaining fields not explicitly set should
    // have the same value as the fields in the given instance.
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Different structs different types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-like struct
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,       //stack
    username: String,   //heap
    email: String,      //heap
    sign_in_count: u64, //stack
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs without any fields
struct AlwaysEqual;
