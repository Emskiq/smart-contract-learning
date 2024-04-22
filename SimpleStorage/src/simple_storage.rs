use serde::Serialize;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, LookupMap};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleStorage {
    my_fav_number: u64,
    list_of_people: Vector<Person>,
    name_to_fav_number: LookupMap<String, u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct Person {
    fav_number: u64,
    name: String,
}

#[near_bindgen]
impl SimpleStorage {
    #[init]
    pub fn new() -> Self {
        SimpleStorage {
            my_fav_number: 0,
            list_of_people: Vector::new(b"list_of_people".to_vec()),
            name_to_fav_number: LookupMap::new(b"map_of_names".to_vec()),
        }
    }

    pub fn store(&mut self, new_fav_number: u64) {
        self.my_fav_number = new_fav_number;
    }

    pub fn retrieve(&self) -> u64 {
        self.my_fav_number
    }

    pub fn add_person(&mut self, name: String, fav_num: u64) {
        let person = Person{
            fav_number: fav_num ,
            name: name.clone()
        };

        self.list_of_people.push(&person);
        self.name_to_fav_number.insert(&name, &fav_num);
    }
}
