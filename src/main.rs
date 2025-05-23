//! Test

#![warn(missing_docs)]

mod foo;

fn main() {
    println!("Hello, world!");
    println!("a {}", foo::bar(1));
}
