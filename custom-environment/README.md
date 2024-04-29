# Custom environment

This contract demonstrates how to use a custom environment in `ink!`.

Because chains built on Substrate can decide on their own which types they want to use for elements such as the chain's block number or account IDs by utilizing the [`pallet-contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts) module,
Ink! provides a trait called [`Environment`](https://docs.rs/ink_env/5.0.0/ink_env/trait.Environment.html), which can be modified based on decisions made by the chain developer.

In most cases, the chain will use the [`DefaultEnvironment`](https://docs.rs/ink_env/5.0.0/ink_env/enum.DefaultEnvironment.html), which is already defined in Ink! and can be freely used.

## Chain-side configuration
In the current example, you need to adjust the pallet contracts configuration in your runtime with the following:

```rust
// In your node's runtime configuration file (runtime.rs)
parameter_types! {
  pub Schedule: pallet_contracts::Schedule<Runtime> = pallet_contracts::Schedule::<Runtime> {
    limits: pallet_contracts::Limits {
      event_topics: 6,
      ..Default::default()
    },
    ..Default::default()
  };
}

impl pallet_contracts::Config for Runtime {
  …
  type Schedule = Schedule;
  …
}
```

### Resources:

- [The official documentation page in Ink!](https://use.ink/basics/chain-environment-types)
- [Official example code](https://github.com/use-ink/ink-examples/tree/main/custom-environment)
- [Env Attribute](https://use.ink/macros-attributes/contract#env-impl-environment)
