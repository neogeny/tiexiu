use std::any::type_name;

fn main() {
    let name = type_name::<i32>();
    println!("The type name is: {}", name); // Output: i32
}

use std::any::type_name_of_val;

fn main() {
    let x = 42.0f64;
    println!("The type of x is: {}", type_name_of_val(&x)); // Output: f64
}
