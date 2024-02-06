# Getting Started
To use Arbiter, you can use the Arbiter CLI to help you manage your projects or, if you feel you don't need any of the CLI features, you can be free to use the [`arbiter-core`](https://crates.io/crates/arbiter-core), `arbiter-engine`, and [`arbiter-bindings`](https://crates.io/crates/arbiter-bindings) crates directly.
You can find more information about these crates in the [Usage](../index.md) section.
The crates (aside from `arbiter-engine` at the moment) are linked to their crates.io pages so you can add them to your project by:
```toml
[dependencies]
arbiter-core = "*" # You can specify a version here if you'd like
arbiter-bindings = "*" # You can specify a version here if you'd like 
arbiter-engine = "*" # You can specify a version here if you'd like
```


# Auditing

The current state of software auditing in the EVM is rapidly evolving. Competitive salaries are attracting top talent to firms like [Spearbit](https://spearbit.com/), [ChainSecurity](https://chainsecurity.com/), and [Trail of Bits](https://www.trailofbits.com/), while open security bounties and competitions like [Code Arena](https://code4rena.com/) are drawing in the best and brightest from around the world. Moreover, the rise of decentralized finance and the value at stake in these EVM-oriented systems have also caught the attention of a collection of black hats. 

As competition in auditing intensifies, auditors will likely need to specialize to stay competitive. With its ability to model the EVM with a high degree of granularity, Arbiter is well-positioned to be leveraged by auditors to develop its tooling and methodologies to stay ahead of the curve.

One such methodology is domain-specific fuzzing. Fuzzing is a testing technique that provides invalid, unexpected, or random data as input to a computer program. The program is then monitored for exceptions such as crashes, failing built-in code assertions, or potential memory leaks. Domain-specific fuzzing in the context of EVM system design involves modeling "normal" system behavior with agents and then playing with different parameters of the system to expose system fragility. 

With its high degree of EVM modeling granularity, Arbiter is well-suited to support and enable domain-specific fuzzing. It can accurately simulate the behavior of the EVM under a wide range of conditions and inputs, providing auditors with a powerful tool for identifying and addressing potential vulnerabilities. Moreover, Arbiter is designed to be highly performant and fast, allowing for efficient and timely auditing processes. This speed and performance make it an even more valuable tool in the rapidly evolving world of software auditing.

