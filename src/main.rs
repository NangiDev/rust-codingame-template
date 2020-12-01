use {{crate_name}}::{advanced_math, math};

extern crate {{crate_name}};

fn main() {
    println!("Hello, {{crate_name}}!");
    println!("Math::add(2,3) = {}", math::add(2, 3));
    println!(
        "Math::passthrough_multiply(2,3) = {}",
        math::passthrough_multiply(2, 3)
    );
    println!(
        "advanced_math::multiply(2,3) = {}",
        advanced_math::multiply(2, 3)
}
