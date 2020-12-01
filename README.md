Project for creating a rust codingame base with the cargo-generate crate. https://github.com/ashleygwilliams/cargo-generate

## Getting started

### Prerequirements
The offical [bundler](https://github.com/slava-sh/rust-bundler) crate from [crates.io](https://crates.io/crates/bundler) has some missing support for Rust 2018 keywords. [Solution and pull requst has been provided](https://github.com/slava-sh/rust-bundler/pull/7)

#### Workaround
Use [Mariomka fork](https://github.com/mariomka/rust-bundler) to manually build the bundler.
```
git clone https://github.com/mariomka/rust-bundler.git
cd rust-bundler
cargo install --path .
```
#### Using bundler from crates.io
```
cargo install bundler
```

### VS-code

if running in vscode just run default build task and a single file will be generated into src/bin/singlefile.rs

```
ctrl+shirt+b
```

This will run tests, build bin {{crate_name}}, and generate the singlefile.rs

### Manually

```
cargo test && cargo build --bin {{crate_name}} && bundle . > output/singlefile.rs
```
