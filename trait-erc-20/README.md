# ERC-20 implementation with trait

Yet another smart contract written in [`ink!`](https://use.ink/how-it-works). This is an example that shows how to use traits (known as _interfaces_ in other languages like C++/Java) in `ink!`.
<br>
It is a relatively straightforward process, especially if you are familiar with Rust's traits – basically, you add one macro: [`#ink::trait_definition`](https://docs.rs/ink_lang/latest/ink_lang/attr.trait_definition.html)
above your trait definition.

This contract is developed by following the [official example](https://github.com/use-ink/ink-examples/tree/main/trait-erc20) and the [official documentation](https://use.ink/basics/trait-definitions).

**Once again, I highly recommend doing the official tutorial steps** to discover the [basics](https://use.ink/basics/contract-template) of the `ink!` smart contract features within the library.

### Resources:

- [The official example code](https://github.com/use-ink/ink-examples/tree/main/trait-erc20)
- [Traits in Ink!](https://use.ink/basics/trait-definitions)
- [ERC-20 explanation](https://www.bitcoin.com/get-started/what-are-erc-20-tokens/)
- [Some other official example contracts](https://use.ink/examples/smart-contracts/)
