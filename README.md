<h1 align="center">manifest-std</h1>
<p align="center">
  <a href="#overview"><img src="https://raw.githubusercontent.com/cosmos/chain-registry/00df6ff89abd382f9efe3d37306c353e2bd8d55c/manifest/images/manifest.png" alt="Lifted Initiative" width="100"/></a>
</p>

This crate generates Rust bindings from `.proto` files using `prost-build` and emits compile‑time `TYPE_URL` constants for selected packages. 


## Requirements

- Rust 1.81

## Building 
```shell
cargo build --release
```

The generated bindings will be located in the `src/gen` directory.
