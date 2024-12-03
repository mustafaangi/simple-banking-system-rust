```markdown
# Rust Banking System

A simple banking system implementation in Rust demonstrating the use of traits and structs.

## Setup

```bash
# Clone the repository
git clone https://github.com/mustafaangi/simple-banking-system-rust.git

# Navigate to project directory
cd banking_system

# Build and run
cargo run
```

## Features

- Create bank accounts with account number and holder name
- Deposit money into accounts
- Withdraw money from accounts
- Check account balances
- Basic error handling for insufficient funds

## Usage

The system implements a basic `Account` trait with the following operations:

```rust
// Create new accounts
let mut account1 = BankAccount::new("1001", "Alice Smith");
let mut account2 = BankAccount::new("1002", "Bob Jones");

// Deposit money
account1.deposit(1000.0);

// Withdraw money
account2.withdraw(200.0);

// Check balance
println!("Balance: ${:.2}", account1.balance());
```

## Project Structure

```
banking_system/
├── Cargo.toml
└── src/
    └── 

main.rs


```

## Implementation Details

- Uses Rust traits for defining account behavior
- Implements basic banking operations using struct methods
- Includes simple error handling for insufficient funds
- Maintains account balance as f64 type
```
