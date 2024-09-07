struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

// implement associated user function
impl User {
    // constructor for new user
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    // deactivate user
    fn deactivate(&mut self) {
        println!("Deactivating account: {}", self.username);
        self.active = false;
    }
    // takes email address and defines new user with derived username
    fn from_email() -> Self {
        let mut email = String::from("rashidrasul@me.com");
        println!("Creating new user from email: {}", email);
        let username = email.split('@').next().unwrap().to_string();
        let uri = String::from("https://rashidrasul.com");
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
} // end of impl block

// create a regular method for the User struct to update_uri
fn update_uri(user: &mut User, new_uri: String) {
    println!("Updating URI: {}", user.username);
    user.uri = new_uri;
}

// invoke the method on an instance of User

// create a new user, print status, deactivate and print status again
fn main() {
    // create an instance of user
    let mut new_user = User::new(
        String::from("rashidrasul"),
        String::from("rashidrasul@me.com"),
        String::from("https://rashidrasul.com"),
    );
    println!("Hello, {}!", new_user.username);

    // print status
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );

    // deactivate the user
    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );

    // create new user from email
    let mut new_user = User::from_email();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );

    // update URI and print
    update_uri(&mut new_user, String::from("https://newsite.com"));
    println!("Updated URI for {}: {}", new_user.username, new_user.uri);
}
