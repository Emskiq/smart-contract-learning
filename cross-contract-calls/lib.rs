#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod cross_contract_calls {
    use ink::codegen::TraitCallBuilder;
    use other_contract::OtherContractRef;

    #[ink(storage)]
    pub struct CrossContractRef {
        other_contract: OtherContractRef,
    }

    impl CrossContractRef {
        #[ink(constructor)]
        pub fn new(other_contract_code_hash: Hash) -> Self {
            let other_contract = OtherContractRef::new(true)
                .code_hash(other_contract_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { other_contract }
        }

        /// Builder way of accessing a contract
        #[ink(constructor)]
        pub fn new_v1() -> Self {
            let other_contract = OtherContractRef::new(true)
                .instantiate_v1()
                .code_hash(Hash::from([0x42;32]))
                .gas_limit(0)
                .endowment(10)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { other_contract }
        }

        #[ink(message)]
        pub fn flip_and_get(&mut self) -> bool {
            self.other_contract.flip();
            self.other_contract.get()
        }

        /// Basic invocation of the other contract via the contract reference. (CallBuilder)
        /// *Note* this will invoke the original `call` (V1) host function, which will be
        /// deprecated in the future.
        #[ink(message)]
        pub fn flip_and_get_v1(&mut self) -> bool {
            // We needed the first include in order for that to work
            let call_builder = self.other_contract.call_mut();

            call_builder.flip().call_v1().invoke();
            call_builder.get().call_v1().invoke()
        }


    }
}
