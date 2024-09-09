use my_library::{public_function, PublicEnum, PublicStruct};

fn main() {
    // Create an instance of PublicStruct
    let public_struct = PublicStruct::new(42, String::from("Private data"));
    public_struct.display();

    // Use the PublicEnum
    let enum_value = PublicEnum::VariantB(100);
    match enum_value {
        // Print appropriate variant on match
        PublicEnum::VariantA => println!("Variant A"),
        PublicEnum::VariantB(num) => println!("Variant B with value: {}", num),
    }

    // Call the public function
    public_function();
}
