#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod proxy {

    #[ink(storage)]
    pub struct Proxy {
        /// The `AccountId` of a contract where any call that does not match a
        /// selector of this contract is forwarded to (like a fallback)
        forward_to: AccountId,

        /// The `AccountId` of a privileged account that can update the
        /// forwarding address. This address is set to the account that
        /// instantiated this contract.
        admin: AccountId,
    }

    impl Proxy {
        /// Constructor that initializes the "fallback" contract that
        /// will be called by default in case of no selector is matched
        #[ink(constructor)]
        pub fn new(default_forward: AccountId) -> Self {
            Self {
                forward_to: default_forward,
                admin: Self::env().caller(),
            }
        }

        /// Changes the `AccountId` of the contract where any call that does
        /// not match a selector of this contract is forwarded to.
        /// # Note
        /// Only one extra message with a well-known selector `@` is allowed.
        #[ink(message, selector = @)]
        pub fn change_forward_address(&mut self, new_address: AccountId) {
            assert_eq!(
                self.env().caller(),
                self.admin,
                "caller {:?} does not have sufficient permissions, only {:?} does",
                self.env().caller(),
                self.admin,
            );
            self.forward_to = new_address;
        }

        /// Fallback message for a contract call that doesn't match any
        /// of the other message selectors.
        /// # Note:
        /// - We allow payable messages here and would forward any optionally supplied
        ///   value as well.
        /// - If the self receiver were `forward(&mut self)` here, this would not
        ///   have any effect whatsoever on the contract we forward to.
        #[ink(message, payable, selector = _)]
        pub fn forward(&self) -> u32 {
            let mut call_flags = ink::env::CallFlags::empty();
            call_flags.set(ink::env::CallFlags::FORWARD_INPUT, true);
            call_flags.set(ink::env::CallFlags::TAIL_CALL, true);

            let _ = ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .call(self.forward_to)
                .transferred_value(self.env().transferred_value())
                .call_flags(call_flags)
                .try_invoke()
                .unwrap_or_else(|err| {
                    panic!(
                        "cross-contract call to {:?} failed due to {:?}",
                        self.forward_to, err
                    )
                });
            unreachable!(
                "the forwarded call will never return since `tail_call` was set"
            );
        }

    }
}
