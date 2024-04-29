# Smart Contracts examples

This repository will keep track of the example contracts, regardless of size or complexity, that I have completed during second step of _my Rust auditor path_ – learning smart contract development and the basics of Rust auditing.

## Starting Point

I began by following [this playlist](https://www.youtube.com/playlist?list=PL7mnz6eNKsYlLiLfhDKtVjdSb59cBZPVL) of video tutorials, which introduced me to [`ink!`](https://use.ink/) and [Substrate](https://docs.substrate.io/tutorials/smart-contracts).
The contracts covered in these tutorials include:

- [Simple Storage](simple-storage)
- [Flipper](flipper)
- [Incrementer](incrementer)
- [ERC-20](erc-20)
- [ERC-20 with Traits](trait-erc-20)
- [Cross contract calls](cross-contract-calls)
- [Upgradeable contracts](upgradeable-contracts)
  - [Proxy pattern](upgradeable-contracts/proxy)
  - [Delegate pattern](upgradeable-contracts/delegate)
  - [Set code hash](upgradeable-contracts/set-code-hash)
- [Custom environment](custom-environment)

While these contracts are simple, they provide a solid foundation in understanding Ink! and the actual interaction with contracts through [`substrate-contracts-node`](https://github.com/paritytech/substrate-contracts-node).
