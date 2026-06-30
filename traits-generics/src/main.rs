use std::fmt::Display;

fn main() {
    println!("=== Rust Trait 与泛型 ===");
    println!("在这里练习：trait 定义与实现、泛型函数、trait bound");
}

fn print_demo<T: Display>(value: T)
where
    T: Animal + Display,
{
    println!("value: {}", value);
}

trait Animal {}

struct Dog {
    name: String,
}

impl Animal for Dog {}
