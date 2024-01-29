# RustAuthChain

## Overview
This project is a simple implementation of a blockchain built with Rust. It's designed to provide a foundational structure for a distributed mapping drone system, specifically tailored for disaster relief efforts. The blockchain uses a Proof of Authority (PoA) consensus mechanism and is intended to run on a ZimaBoard Single Board Computer (SBC).

## Features
- **Custom Blockchain Implementation**: A simplistic blockchain structure with basic functionalities.
- **Proof of Authority (PoA) Consensus**: A simple consensus mechanism suitable for private networks.
- **Rust Macros for Block Creation**: Simplified block creation using Rust macros.
- **Pretty Display of Blocks**: Utilizes `prettytable-rs` for a visually appealing presentation of blockchain data.

## Getting Started

### Prerequisites
- Rust and Cargo (latest stable version)
- Git (for cloning the repository)

### Installation

1. **Clone the repository**

    ```sh
    git clone https://github.com/dmarcr1997/RustAuthChain.git
    cd RustAuthChain/rusty-chain
    ```

2. **Build the project**

    ```sh
    cargo build
    ```

### Usage

- Run the project:

    ```sh
    cargo run
    ```

- The blockchain will initialize, create some blocks, and display them in a formatted table.

## Project Structure

- `src/main.rs`: The main entry point of the application.
- `src/block.rs`: Defines the structure of a block and its associated methods.
- `src/blockchain.rs`: Defines the blockchain structure and its methods, including block addition and display functionality.

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Damon Rocha - [@DamonMRocha](https://twitter.com/DamonMRocha) - dmarcr1997@gmail.com


## Acknowledgments

- [prettytable-rs](https://crates.io/crates/prettytable) for the table formatting in the display function.
- [sha2](https://crates.io/crates/sha2) and [hex](https://crates.io/crates/hex) for cryptographic functions.
