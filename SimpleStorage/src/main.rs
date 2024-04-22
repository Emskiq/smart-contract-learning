extern crate near_sdk;
extern crate borsh;
extern crate serde;

mod simple_storage;

use simple_storage::SimpleStorage;

fn main() {
    let mut simple_storage = SimpleStorage::new();
    simple_storage.store(69);

    println!("Hello, world!");
}
