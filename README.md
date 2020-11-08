# zv64

A simple Rust implementation of the RV64I base instruction set architecture

## Features

#### Opcodes

- 0x13 (addi)
- 0x33 (add)

## Usage

Ensure you have [Rust & Cargo](https://www.rust-lang.org) installed before you attempt to build zv64

Clone the repository
```
$ git clone https://github.com/zhooda/zv64.git
$ cd zv64
```

Build and run using Cargo
```
# cargo run --release -- <path-to-rs64i-binary>
$ cargo run --release -- add-addi.bin
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
