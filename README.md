<br />
<p align="center">
  <a href="https://arbitrum.io/">
    <img src="https://arbitrum.io/wp-content/uploads/2023/08/stylus-thumbnail-1000.png" alt="Logo" width="100%">
  </a>

  <h3 align="center">The Stylus SDK</h3>

  <p align="center">
    <a href="https://developer.arbitrum.io/"><strong>Rust contracts on Arbitrum »</strong></a>
    <br />
  </p>
</p>

## Overview

The Stylus SDK enables smart contract developers to write programs for **Arbitrum chains** written in the [Rust](https://www.rust-lang.org/tools/install) programming language. Stylus programs are compiled to [WebAssembly](https://webassembly.org/) and can then deployed onchain to be executed alongside Solidity smart contracts. Stylus programs are not only orders of magnitude cheaper and faster but also enable what was thought to be previously impossible for WebAssembly: **EVM-interoperability**.

For information about deploying Rust smart contracts, see the [Cargo Stylus CLI Tool](CargoStylus). For more information about Stylus, see [Stylus: A Gentle Introduction](https://docs.arbitrum.io/stylus/stylus-gentle-introduction). For a simpler intro to Stylus Rust development, see the [Quick Start guide](https://docs.arbitrum.io/stylus/stylus-quickstart).

Comprehensive documentation on the Rust SDK can be found [here](https://docs.arbitrum.io/stylus/rust-sdk-guide).

## Feature highlights

The SDK makes it easy to develop Ethereum ABI-equivalent Stylus contracts in Rust. It provides a full suite of types and shortcuts that abstract away the details of Ethereum's storage layout, making it easy to _just write Rust_.

Some of the features available in the SDK include, but are not limited to:

- **Generic**, storage-backed Rust types for programming **Ethereum-equivalent** smart contracts
- Simple macros for writing Solidity structs and **entrypoints** that get converted to SDK-types internally
- Powerful **primitive types** backed by the feature-rich [alloy-rs/crate](https://github.com/alloy-rs/core)

Rust programs implemented with the Stylus SDK can **call and be called** by Solidity smart contracts through Ethereum ABIs and also share the same storage layout.

```rust
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Generate Solidity-equivalent, Rust structs backed by storage.
sol_storage! {
  #[entrypoint]
  pub struct Counter {
    uint256 number;
  }
}

#[external]
impl Counter {
  // Gets the number value from storage.
  pub fn number(&self) -> Result<U256, Vec<u8>> {
    Ok(self.number.get())
  }
  // Sets a number in storage to a user-specified value.
  pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
    self.number.set(new_number);
    Ok(())
  }
}
```

Additionally, the Stylus SDK supports `#[no_std]` for contracts that wish to opt out of the standard library. In fact, the entire SDK is available from `#[no_std]`, so no special feature flag is required. This can be helpful for reducing binary size, and may be preferable in pure-compute use cases like cryptography.

Most users will want to use the standard library, which is available since the Stylus VM supports `rustc`'s `wasm32-unknown-unknown` target triple. In the future we may add `wasm32-wasi` too, along with floating point and SIMD, which the Stylus VM does not yet support.

## Don't know Rust?

The Stylus VM supports more than just Rust. In fact, any programming language that compiles down to WebAssembly could in principle be deployed to Stylus-enabled chains. The table below includes the official ports of the SDK, with more coming soon.

| Repo             | Use cases                   | License           |
|:-----------------|:----------------------------|:------------------|
| [Rust SDK][Rust] | Everything!                 | Apache 2.0 or MIT |
| [C/C++ SDK][C]   | Cryptography and algorithms | Apache 2.0 or MIT |
| [Bf SDK][Bf]     | Educational                 | Apache 2.0 or MIT |
| [Cargo Stylus][CargoStylus]     | Deploying Stylus programs | Apache 2.0 or MIT |

Want to write your own? [Join us in the `#stylus` channel on discord][discord]!

[Rust]: https://github.com/OffchainLabs/stylus-sdk-rs
[C]: https://github.com/OffchainLabs/stylus-sdk-c
[Bf]: https://github.com/OffchainLabs/stylus-sdk-bf

[discord]: https://discord.com/invite/5KE54JwyTs

## Developing Stylus Programs

The Stylus SDK is just one of the building blocks in creating and deploying WebAssebmly programs to Arbitrum chains. To create a new Stylus project from a hello-world example and deploy it onchain, check out some of our tools below:

| Repo             | Use cases                   | License           |
|:-----------------|:----------------------------|:------------------|
| [Stylus Hello World][HelloWorld]     | Rust Stylus starter template | Apache 2.0 or MIT |
| [Cargo Stylus CLI][CargoStylus]     | Deploying Stylus programs | Apache 2.0 or MIT |

[HelloWorld]: https://github.com/OffchainLabs/stylus-hello-world
[CargoStylus]: https://github.com/OffchainLabs/cargo-stylus

## License

&copy; 2022-2023 Offchain Labs, Inc.

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([licenses/Apache-2.0](licenses/Apache-2.0))
- [MIT license](https://opensource.org/licenses/MIT) ([licenses/MIT](licenses/MIT))

at your option.

The [SPDX](https://spdx.dev) license identifier for this project is `MIT OR Apache-2.0`.
