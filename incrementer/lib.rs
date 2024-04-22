#![cfg_attr(not(feature="std"), no_std)]


#[ink::contract]
mod incrementer {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Incrementer {
        value: u32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_val: u32) -> Self {
            Self{value : init_val}
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self{value : 0}
        }

        #[ink(message)]
        pub fn inc(&mut self, by: u32) {
            self.value = by;
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }
    }
}
