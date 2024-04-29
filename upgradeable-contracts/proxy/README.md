# Proxy

This contract implements the [_Proxy pattern_](https://medium.com/geekculture/proxy-contract-7f05ff50a2c7) or [_Proxy forwarding_](https://use.ink/basics/upgradeable-contracts/#proxy-forwarding) in `ink!`.

If you are familiar with that pattern, the interesting part will likely be in the code itself and, most importantly, the [`#ink(selector = ...)`](https://use.ink/macros-attributes/selector/) attribute macro.
<br>
By default, `ink!` creates a selector for each message and constructor. Using this attribute, it is possible to specify a concrete dispatch selector for the flagged function.

There is a lot of good information regarding the `selector` attribute to discover in the [official documentation](https://use.ink/macros-attributes/selector/).

### Resources:

- [The official documentation page in Ink!](https://use.ink/basics/upgradeable-contracts/#proxy-forwarding)
- [Proxy pattern explanation](https://blog.openzeppelin.com/proxy-patterns)
- [`selector` Macro attribute](https://use.ink/macros-attributes/selector/)
- [Issue with the Wildcard selector attribute](https://github.com/use-ink/ink/issues/1676) - probably that is good information regarding auditing 🤔.

---

> **Note:** There wasn't any example code and tests, so I leave it up to you to write the tests 😁
