use json_derive::json;

#[derive(json)]
struct Person {
    name: String,
    age: u128,
}

fn main() {
    println!("Hello, Derive");
}