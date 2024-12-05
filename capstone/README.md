# Capstone Project: Solana-Based Gifting Platform "Solwish"

This repository contains the code for a Solana-based gifting platform that allows users to send and receive tokenized gifts securely on the blockchain. The program utilizes Solana's Anchor framework and integrates various token-related functionalities.

## Program Address

The program is deployed on Solana Devnet with the following address:

```
5CSHVvjWKynHo2kkCFRTVJhJ1fg1k2FR9RSDzQa38VMb
```

## Features

- **Gift Creation**: Allows users to create a gift with a unique token mint.
- **Secure Transfer**: Ensures only the intended recipient can access the gift.
- **On-Chain Validation**: Verifies the gift's metadata and ownership using Solana's Program Derived Addresses (PDAs).
- **Dynamic Mint Selection**: Supports handling multiple token mints stored in an account.
- **Collection Management**: Creates and manages Metaplex core NFT collections.
- **Token Minting**: Mints and transfers NFTs to recipients.
- **Unpacking Gifts**: Transfers all deposited tokens to the recipient upon validation.

## File Structure

```plaintext
capstone/
├── src/
│   ├── instructions/
│   │   ├── create_gift.rs  
│   │   ├── unpack.rs    
│   │   ├── add_token.rs
│   │   ├── burn.rs
│   │   ├── create_collection.rs
│   │   ├── initalize.rs 
│   │   ├── mint_and_send_NFT.rs 
│   ├── state/
│   │   ├── mod.rs
│   │   ├── gift_config.rs
│   ├── errors.rs
│   ├── lib.rs             
├── Cargo.toml             
├── Anchor.toml             
└── README.md              
```

## Installation and Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://book.anchor-lang.com/chapter_2/installation.html)

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/Se76/Q4_Sol_Se76.git
   cd capstone
   ```

2. Install dependencies:

   ```bash
   anchor build
   ```

3. Deploy the program to your Solana Devnet:

   ```bash
   anchor deploy
   ```

4. Update the `Anchor.toml` file with the deployed program ID.

5. Build the project using Cargo:

   ```bash
   cargo build
   ```

6. Add additional dependencies using Yarn:

   ```bash
   yarn add '@solana/spl-token'
   yarn add '@metaplex-foundation/mpl-core'
   yarn add '@metaplex-foundation/umi'
   ```

7. Use the CLI command `touch id.json` to create a new file named `id.json` and save your private key there for local usage.

## Functionality Overview

The following key functionalities are implemented in the program:

### 1. Initialization

Initializes the program with a greeting message.

```rust
pub fn initialize(ctx: Context<Initialize>, greetings: String) -> Result<()> {
    ctx.accounts.initialize(greetings, &ctx.bumps)?;
    Ok(())
}
```

### 2. Add Token

Adds a token with a specific amount and decimal value.

```rust
pub fn add_token(ctx: Context<AddToken>, amount_of_token: u64, decimals: u8) -> Result<()> {
    ctx.accounts.add_token(amount_of_token, decimals)?;
    Ok(())
}
```

### 3. Create Collection

Creates a collection for organizing NFTs.

```rust
pub fn create_collection(ctx: Context<CollectionAccounts>) -> Result<()> {
    ctx.accounts.create_collection()?; 
    Ok(())
}
```

### 4. Mint and Send NFT

Mints an NFT and sends it to the recipient.

```rust

### 6. Unpack

Unpacks a gift by transferring all deposited tokens to the recipient.

```rust
pub fn unpack(ctx: Context<Unpack>) -> Result<()> {
    ctx.accounts.unpack()?;
    Ok(())
}
```

## Tested Transactions

### final_capstone

**Transaction signature:**  3SdjkK1nYMoG1aJzZ5iN7ohshYVFA2qDb5dk8aUR1V5Vm5UYUanE8ztt5njqTzAkvDZaZopGxbKX4nXVNFndYuso

- ✔ Program and GiftConfig are initialized (2732ms)

Your mint is: DVeRGVFx17JvhVpE1SfKtfpForEoVJKKdFtv6DQcCETy  
Your ata is: ELV6aW9QK2ANvmNhadtKWPyYxqfYQ95CqMK9foDvtJ5B  
Your mint txid: 66BCwXHm6DRusm7RDYPrDKjaBGD8ZS2Jzp2GMGFQcmy5JJfSzV7YgnzkxkSN3YhA7dvEMtivoZG3d9Z57sokZyFK  
Your transaction signature xJBaAm7aSwr9Jma9vrWK9VNzR322xX4Ewm1FnSRHgLRiVWhWUVZRKvQVMqVD8DsWbVodgWRFkKba79NcZGJjvzo

- ✔ Token 1 is created and added to the gift (4638ms)

Your mint is: HfjMmmsmQaSUyN5sfPpjK255GWvezPiHdp2tzWdfp1Fz  
Your ata is: 4bi4WX7HBBq9NkcUkHwULkPkRT195MhvkYWtELQQkbKN  
Your mint txid: 4kSbLi5UTvCGLjuC2DQxPJqEoAfXkwg8MZTPPC7peSyVpAu5yhGZwgRShUN1PYfcLRkcakfVZ9WiiaPPrgfdpNLr  
Your transaction signature GivMCNDi1PdUfHdgASbh9zZWkPup5QBQcb6S1P6oqhLXkwSezYYr3knhNzqMpaddfiqymY6Y72ZRfrU84R2VU3J

- ✔ Token 2 is created and added to the gift (1506ms)

Your transaction signature WsvguDEyg3Dkz4nq4uaN4ZzBiz51ytcxvoUByXVxJfBshjjwDEEHPyv4f2nL6gL8wHAKNRkKYFusU9zQFCwuU97

- ✔ Collection created (338ms)

Your transaction signature 3bWyDB6mosZsLBeDooz5XstKurUNbNDKLJV5KwCY8jjZxzvouaY71LX1dd6JewDE2XCArzJsBv41WZ1DGRv4UcvP

NFT 44HXyR5BdHpbZyq6ewkZXnwASbmyZeWCtn1wmSDnjrxF

- ✔ NFT minted and sent to receiver (366ms)

Your transaction signature 3bsc7BEJrhrGHoicmVN2PRdYakBHZ924DbEbzCE1YQwA2hBfx5b3Aa48w4ie3reEtdYB1QCad4nhLf6UXc7cdBwE

- ✔ NFT burnt (362ms)

Your transaction signature RXb6yDJtPHM4GghKEWSm7z3GJEcKGHdicDu2fXKpF8DoRt1vvAbnx3G6n7RhgnYDPzmVkCP18GnzPSEga7BgEmB

- ✔ Asset 1 unpacked (3637ms)

Your transaction signature 4euerUTTdP7XvXC4m9HyN46kR5jSCABo1oB4ppPKesSi7BWPeULCFhBp72wcZu5srxWGuctmZWDbzWxo6zQPBzTE

- ✔ Asset 2 unpacked (270ms)

