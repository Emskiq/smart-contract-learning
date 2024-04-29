#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incrementer {

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
            self.value = self.value.checked_add(by).unwrap();
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }
    }
}
