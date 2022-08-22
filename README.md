# Nairobi Expressway Payment System

## About this repository

This repository is a Web3 payment system designed for use in the recently commissioned Nairobi expressway. This project utilises [NEAR Rust SDK](https://www.near-sdk.io/) for the smart contracts to compile Rust to Web Assembly (WASM), the low-level language used by the NEAR platform. Toll rates were obtained from the Nairobi expressway [website](https://nairobiexpressway.ke/).

## Getting started

To get started:

1. Clone this repository

2. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)

3. Test the contract 

    `cargo test -- --nocapture`

4. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/develop/prerequisites)
* [Rust SDK Book](https://www.near-sdk.io/)
