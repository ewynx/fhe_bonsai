# Bonsai x FHE

A small demo of running a homomorphic multiplication (using BFV) with Bonsai. 

The FHE library and the example are [here](https://github.com/tlepoint/fhe.rs/tree/main) and [here](https://github.com/tlepoint/fhe.rs/blob/main/crates/fhe/README.md). The Risc0 zkVM template that was used to start this project is described [here](https://github.com/risc0/risc0/tree/main/risc0/cargo-risczero).

For this repo a fork of the FHE library is used which uses a compatible Rust version for the zkVM. 

## Quick Start

Init and update submodule:
```
git submodule init
git submodule update
```

To build all methods and execute the method within the zkVM, run the following command:

```bash
cargo run
```

To run in developer mode, without generating valid proofs (much faster):

```bash
RISC0_DEV_MODE=1 cargo run
```

### Running proofs remotely on Bonsai

If you have access to the URL and API key to Bonsai you can run your proofs
remotely. To prove in Bonsai mode, invoke `cargo run` with two additional
environment variables:

```bash
BONSAI_API_KEY="YOUR_API_KEY" BONSAI_API_URL="BONSAI_URL" cargo run
```


## Resources

- fhe.rs: https://github.com/tlepoint/fhe.rs
- Risc0 zkVM: https://github.com/risc0/risc0
- Bonsai access: https://bonsai.xyz/apply
