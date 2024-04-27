# Cross Contract Usage Example

Smart contract written in [`ink!`](https://use.ink/how-it-works). This is an example that demonstrates how you can use other Smart Contracts in your own implementation in `ink!`.

There are two ways to enable you to use SCs in your own one:
- [`ContractRefs`](https://use.ink/basics/cross-contract-calling#contract-references)
- [`CreateBuilder`](https://use.ink/basics/cross-contract-calling#createbuilder) and [`CallBuilder`](https://use.ink/basics/cross-contract-calling#callbuilder)

_Check the comments in the code example to see both ways in action._

This contract is developed by following the [official example](https://github.com/use-ink/ink-examples/tree/main/cross-contract-calls) and the [official documentation](https://use.ink/basics/cross-contract-calling).

**Once again, I highly recommend doing the official tutorial steps** to discover the [basics](https://use.ink/basics/contract-template) of the `ink!` smart contract features within the library.

---

> **Note:** The example commands weren't working for me until I actually executed them by adding the `--execute` argument.

### Resources:

- [The official example code](https://github.com/use-ink/ink-examples/tree/main/trait-erc20)
- [The official documentation page in Ink!](https://use.ink/basics/cross-contract-calling)
