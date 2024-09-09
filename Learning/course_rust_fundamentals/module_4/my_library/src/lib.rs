// Public struct with a private field
pub struct PublicStruct {
    pub field: i32,
    private_field: String, // Private field, not accessible outside the module
}

impl PublicStruct {
    // Constructor: Public function to create a new PublicStruct
    pub fn new(field: i32, private_field: String) -> Self {
        PublicStruct {
            field,
            private_field,
        }
    }

    // Public method that uses a private field
    pub fn display(&self) {
        println!(
            "Field: {}, Private Field: {}",
            self.field, self.private_field
        );
    }
}

// Private struct and its implementation
struct PrivateStruct {
    data: String,
}

impl PrivateStruct {
    // PrivateStruct constructor
    fn new(data: String) -> Self {
        PrivateStruct { data }
    }

    fn show_data(&self) {
        println!("Private data: {}", self.data);
    }
}

// Public enum
pub enum PublicEnum {
    VariantA,
    VariantB(i32),
}

// Private enum
enum PrivateEnum {
    Hidden,
    Secret(String),
}

// Public function
pub fn public_function() {
    println!("This is a public function");
    // Calling a private function from within the library
    private_function();
}

// Private function
fn private_function() {
    println!("This is a private function");
    // Creating and using private struct
    let private = PrivateStruct::new(String::from("Secret data"));
    private.show_data();
}

// Public function demonstrating usage of public enum
pub fn use_public_enum(value: PublicEnum) {
    match value {
        PublicEnum::VariantA => println!("Variant A"),
        PublicEnum::VariantB(num) => println!("Variant B with value: {}", num),
    }
}

// Private function demonstrating usage of private enum
fn use_private_enum(value: PrivateEnum) {
    match value {
        PrivateEnum::Hidden => println!("Hidden variant"),
        PrivateEnum::Secret(secret) => println!("Secret: {}", secret),
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn test_public_struct_display() {
        let public_struct = PublicStruct::new(42, String::from("Private data"));
        // This requires capturing the output or modifying the code to be testable
        // For simplicity, we will assume this method works as expected if no panic occurs
        public_struct.display();
    }

    #[test]
    fn test_public_enum() {
        let variant_a = PublicEnum::VariantA;
        let variant_b = PublicEnum::VariantB(100);

        // Using `use_public_enum` function
        use_public_enum(variant_a);
        use_public_enum(variant_b);
    }

    #[test]
    fn test_public_function() {
        // Calling the public function to ensure it works without panicking
        public_function();
    }

    // Note: Private items are not accessible directly
    // Therefore, we cannot test `PrivateStruct`, `PrivateEnum`, or `private_function` directly.
}
