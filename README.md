# Zero To Production In Rust

## Installation

### Pre-requisites

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

### Additional for MacOS

```bash
brew install michaeleisel/zld/zld
```

## Development

### Coding and watching
    
```bash
cargo watch -x check -x test -x run
```

### Adding dependency
```bash
cargo add <dependency>
```

Or if you want to add a dev dependency:
```bash
cargo add <dependency> --dev
```

Or you can add a dependency manually by adding under [dependencies] in our Cargo.toml

### Tests

```bash
cargo test
```

### Code Coverage

```bash
cargo install cargo-tarpaulin
```

```bash
cargo tarpaulin --ignore-tests
```


### Linting

```bash
cargo install cargo-clippy
```

```bash
cargo clippy
```

### Formatting

```bash
cargo install rustfmt
```

```bash
cargo fmt
```

### Secure Coding

```bash
cargo install cargo-audit
```

```bash
cargo audit
```

There are a few other tools that can be installed to help with security:

```bash
cargo install cargo-geiger
cargo install cargo-outdated
```
