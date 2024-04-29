#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod delegate {
    use ink::storage::{Mapping, Lazy};
    use ink::storage::traits::ManualKey;

    use ink::env::{DefaultEnvironment, CallFlags};
    use ink::env::call::{build_call, ExecutionInput, Selector};

    #[ink(storage)]
    pub struct Delegator {
        addresses: Mapping<AccountId, i32, ManualKey<0x23>>,
        counter: i32,
        
        /// The contract that we are delegating to.
        delegate_to: Lazy<Hash>
    }

    impl Delegator {
        #[ink(constructor)]
        pub fn new(init_value: i32, hash: Hash) -> Self {
            // Initialize the hash of the contract to delegate to.
            // Adds a delegate dependency lock, ensuring that the delegated to code cannot be removed.
            let mut delegatee = Lazy::new();
            delegatee.set(&hash);
            Self::env().lock_delegate_dependency(&hash);

            Self {
                addresses: Mapping::default(),
                counter: init_value,
                delegate_to: delegatee,
            }
        }

        /// Increment the current value using delegate call.
        #[ink(message)]
        pub fn inc_delegate(&self) {
            let selector = ink::selector_bytes!("inc");
            let _ = build_call::<DefaultEnvironment>()
                .delegate(self.delegate_to())
                // We specify `CallFlags::TAIL_CALL` to use the delegatee last memory frame
                // as the end of the execution cycle.
                // So any mutations to `Packed` types, made by delegatee,
                // will be flushed to storage.
                //
                // If we don't specify this flag, the storage state before the delegate
                // call will be flushed to storage instead.
                .call_flags(CallFlags::TAIL_CALL)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<()>()
                .try_invoke();
        }

        /// Adds entry to `addresses` using delegate call.
        /// Note that we don't need `set_tail_call(true)` flag
        /// because `Mapping` updates the storage instantly on-demand.
        #[ink(message)]
        pub fn add_entry_delegate(&mut self) {
            let selector = ink::selector_bytes!("append_address_value");
            let _ = build_call::<DefaultEnvironment>()
                .delegate(self.delegate_to())
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<()>()
                .try_invoke();
        }

        /// Returns the current value of the counter.
        #[ink(message)]
        pub fn get_counter(&self) -> i32 {
            self.counter
        }

        /// Returns the current value of the address.
        #[ink(message)]
        pub fn get_value(&self, address: AccountId) -> (AccountId, Option<i32>) {
            (self.env().caller(), self.addresses.get(address))
        }

        fn delegate_to(&self) -> Hash {
            self.delegate_to
                .get()
                .expect("delegate_to always has a value")
        }
    }

}
