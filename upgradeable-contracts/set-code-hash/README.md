# Set Code Hash

This contract is demonstating the usage of [`set_code_hash`](https://docs.rs/ink_env/latest/ink_env/fn.set_code_hash.html)

Because `ink!` is following the [Substrate's runtime upgradeability philosophy](https://docs.substrate.io/maintain/runtime-upgrades/) - it provides an easy way update your contract code via this special `set_code_hash` function.

However there are some important things that you need to take as consideration when using the `set_code_hash` function:

- This function effectively replaces the code which is executed for the contract address.
- The other contract needs to be deployed on-chain.
- State is stored in the storage of the originally instantiated contract.
- You **should not** change the order in which the contract state variables are declared, nor their type! Take a look at [the official documentation](https://use.ink/basics/upgradeable-contracts/#storage-compatibility) for more info.


### Resources:

- [The official documentation page in Ink!](https://use.ink/basics/upgradeable-contracts/#replacing-contract-code-with-set_code_hash)
- [Substrate runtime upgrades](https://docs.substrate.io/maintain/runtime-upgrades/)
- [Set Code Hash documentation](https://docs.rs/ink_env/latest/ink_env/fn.set_code_hash.html)
