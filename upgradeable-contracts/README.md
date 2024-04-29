# Upgradeable contracts

This is a list of different patterns that implement the feature of upgrading your contract. 
Smart contracts on the blockchains are immutable by default. Once you create them, there is no way to alter them, effectively acting as an unbreakable contract among participants.

However, for some scenarios, it is desirable to be able to modify them, for example: critical bug fixes, new features.

Smart contracts are commonly upgraded using software architecture patterns like 
the [_Proxy Pattern_](proxy), [_Delegator pattern_](delegate), and one specific way for **Substrate** by [_setting the code hash_](set-code-hash).
