# Blockchain Data Structure

A comprehensive blockchain implementation in Rust featuring transaction processing, proof-of-work mining, and validation mechanisms.

## Overview

This project implements a complete blockchain data structure with the following core components:
- **Block Structure**: Individual blocks containing transactions and metadata
- **Blockchain**: A validated chain of blocks with transaction validation
- **Transaction System**: Support for inputs, outputs, and coinbase transactions
- **Mining Algorithm**: Proof-of-work consensus mechanism with adjustable difficulty
- **Cryptographic Hashing**: SHA-256 based hashing for security

## Features

- **Transaction Processing**: Full support for UTXO (Unspent Transaction Output) model
- **Mining**: Proof-of-work algorithm with nonce iteration and difficulty adjustment
- **Validation**: Comprehensive block and transaction validation
- **Coinbase Transactions**: Mining rewards and fee collection
- **Double Spend Prevention**: Input tracking and validation
- **Genesis Block**: Special handling for the first block in the chain

## Architecture

### Core Components

- **`Block`**: Contains index, timestamp, hash, previous block hash, nonce, transactions, and difficulty
- **`Blockchain`**: Manages the chain of blocks and unspent transaction outputs
- **`Transaction`**: Represents value transfers with inputs and outputs
- **`Hashable`**: Trait providing SHA-256 hashing functionality

### Key Files

- `src/block.rs` - Block structure and mining implementation
- `src/blockchain.rs` - Blockchain management and validation logic
- `src/transaction.rs` - Transaction structure and processing
- `src/hashable.rs` - Cryptographic hashing trait
- `src/lib.rs` - Core utility functions and type definitions
- `src/main.rs` - Example usage and demonstration

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Setup

```bash
git clone https://github.com/your-username/blockchain-data-structure.git
cd blockchain-data-structure
cargo build --release
```

## Usage

### Running the Example

```bash
cargo run
```

This will execute the demonstration in `main.rs` which:
1. Creates a genesis block with initial transactions
2. Mines the genesis block
3. Creates and mines a second block with additional transactions
4. Validates the entire blockchain

### Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
blockchainlib = { path = "path/to/blockchain-data-structure" }
```

Example usage:

```rust
use blockchainlib::*;

// Create a new blockchain
let mut blockchain = Blockchain::new();

// Create and mine a genesis block
let mut genesis = Block::new(
    0,                          // index
    now(),                      // timestamp
    vec![0; 32],               // previous hash (genesis)
    vec![/* transactions */],   // transactions
    0x000fffffffffffff         // difficulty
);

genesis.mine();
blockchain.update_with_block(genesis).expect("Failed to add genesis block");
```

## API Reference

### Block

- `new()` - Create a new block
- `mine()` - Mine the block using proof-of-work
- `hash()` - Calculate the block's hash

### Blockchain

- `new()` - Initialize an empty blockchain
- `update_with_block()` - Add and validate a new block

### Transaction

- `input_value()` - Sum of all input values
- `output_value()` - Sum of all output values
- `is_coinbase()` - Check if transaction is a coinbase transaction

## Mining

The mining process uses a proof-of-work algorithm:

1. Increment the nonce value
2. Calculate the block hash
3. Check if hash meets difficulty requirement
4. Repeat until valid hash is found

Difficulty is represented as a 128-bit number where lower values require more computational work.

## Validation

The blockchain validates:

- **Block Index**: Sequential block numbering
- **Hash Difficulty**: Proof-of-work validation
- **Timestamp**: Chronological ordering
- **Previous Hash**: Chain integrity
- **Transactions**: Input/output validation and double-spend prevention
- **Coinbase**: Mining reward validation

## Dependencies

- `hex` (0.4.3) - Hexadecimal encoding/decoding
- `crypto-hash` (0.3.4) - SHA-256 cryptographic hashing

## Testing

```bash
cargo test
```

## Building

```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release
```

## License

This project is open source and available under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Timestamp utility function adapted from Stack Overflow
- Implementation follows standard blockchain principles and UTXO model