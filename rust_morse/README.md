## MORSECODE in Rust
[![Build and Test](https://github.com/mathijsglazema/MorsecodeESP/actions/workflows/build-and-test.yaml/badge.svg)](https://github.com/mathijsglazema/MorsecodeESP/actions/workflows/build-and-test.yaml)

## How to get setup
1. Install Rust for your platform
[Rust](https://www.rust-lang.org/tools/install)

2. Clone the repo

3. Install the dependencies: 
```bash
rustup target add armv7-unknown-linux-gnueabihf \
sudo apt-get update && sudo apt-get install build-essential
```

4. Install the compiler: `sudo apt-get install gcc-arm-linux-gnueabihf`

5. Build the project: `cargo build`