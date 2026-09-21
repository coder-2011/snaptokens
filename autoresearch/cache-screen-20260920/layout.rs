use std::{collections::HashMap, mem::size_of};
fn main() {
    println!("String {}", size_of::<String>());
    println!("Box<str> {}", size_of::<Box<str>>());
    println!("(String, Vec<u32>) {}", size_of::<(String, Vec<u32>)>());
    println!("(Box<str>, Vec<u32>) {}", size_of::<(Box<str>, Vec<u32>)>());
    let map: HashMap<String, Vec<u32>> = HashMap::new();
    println!("HashMap empty capacity {}", map.capacity());
}
