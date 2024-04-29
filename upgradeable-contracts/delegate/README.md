# Delegator

This contract implements another aspect of the [_Proxy pattern_](https://medium.com/geekculture/proxy-contract-7f05ff50a2c7) - the [_Delegator_](https://use.ink/basics/upgradeable-contracts/#delegating-execution-to-foreign-contract-code-with-delegate_call) in `ink!`.

It is similar to the [_Proxy_](proxy), and the key takeaways while implementing (or copy and pasting 😁) this contract are:

- Delegates any call that does not match a selector of itself to another contract.
- Code is required to be uploaded on-chain but is not required to be instantiated.
- State is stored in the storage of the original contract which submits the call.
- Storage layout must be identical between both contract codes.
- [`CallBuilder`](https://docs.rs/ink_e2e/latest/ink_e2e/struct.CallBuilder.html)
- [Delegating Dependency Locks](https://use.ink/basics/upgradeable-contracts/#delegate-dependency-locks)

### Resources:

- [The official documentation page in Ink!](https://use.ink/basics/upgradeable-contracts/#delegating-execution-to-foreign-contract-code-with-delegate_call)
- [IIP-2](https://github.com/use-ink/ink/issues/1676) - probably that is good information regarding auditing 🤔.
