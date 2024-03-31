# DeFi Project: Create Cryptocurrency

In this project, I've demonstrated how to create and manage a cryptocurrency token on the Solana blockchain. I've used the Solana CLI and the SPL Token program to showcase the entire process.

## Getting Started

## Concepts Used

- **Solana CLI**: A tool I used for interacting with the Solana blockchain.
- **Public and Secret Keys**: Keys that identify and secure my wallet.
- **SPL Token Program**: The standard I followed to create and manage my tokens.
- **Airdrop**: How I got free SOL for testing.
- **Minting**: The process of creating new tokens to add to my token's supply.
- **Burning**: How I permanently removed tokens from the supply.
- **Transferring Tokens**: How I sent tokens from my wallet to another.

### Prerequisites

- You need to have the Solana CLI installed on your machine.
- Node.js is optional but recommended for advanced interactions.

### Setting Up My Wallet

1. **Creating a New Wallet**
   I used the following command to create a new wallet:

```bash
solana-keygen new
```

I used `--force` to overwrite if I already had an existing wallet.

2. **Getting My Public Key**
   To view my public key, I used:

```bash
solana-keygen pubkey
```

3. **Checking My SOL Balance**
   To check the balance of my wallet, I ran:

```bash
solana balance --url devnet
```

4. **Using Solana Explorer**
   I checked my balance and transactions on [Solana Explorer](https://explorer.solana.com/?cluster=devnet).

### Airdrop SOL

1. **Requesting Airdrop**
   To get some SOL for testing, I requested an airdrop:

```bash
solana airdrop 2 <public-key> --url devnet
```

### Creating a Token

1. **Creating My Token**
   I created my token with the following command:

```bash
spl-token create-token --url devnet
```

### Minting Tokens

1. **Creating a Token Account**
   I needed a token account to hold my tokens, so I created one:

```bash
spl-token create-account <token-address> --url devnet
```

2. **Minting New Tokens**
   To increase the supply of my token, I minted more:

```bash
spl-token mint <token-address> 1000 --url devnet
```

3. **Checking Token Balance**
   To see how many tokens I had, I checked the balance:

```bash
spl-token balance <token-address> --url devnet
```

4. **Viewing Token Supply**
   To view the total supply of my tokens, I used:

```bash
spl-token supply <token-address> --url devnet
```

### Disabling Minting

1. **Disabling Further Minting**
   To stop minting new tokens permanently, I disabled minting:

```bash
spl-token authorize <token-address> mint --disable --url devnet
```

### Encountering an Error

When trying to mint more tokens after disabling minting, I encountered the following error:

```bash
Error: Client(Error { request: Some(SendTransaction), kind: RpcError(RpcResponseError { code: -32002, message: "Transaction simulation failed: Error processing Instruction 0: custom program error: 0x5", data: SendTransactionPreflightFailure(RpcSimulateTransactionResult { err: Some(InstructionError(0, Custom(5))), logs: Some(["Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]", "Program log: Instruction: MintToChecked", "Program log: Error: the total supply of this token is fixed", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4147 of 4147 compute units", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA failed: custom program error: 0x5"]), accounts: None, units_consumed: Some(4147), return_data: None, inner_instructions: None }) }) })
```

### Burning Tokens

1. **Burning Tokens**
   To reduce the supply of my tokens, I burned some:

```bash
spl-token burn <token-account-address> 100 --url devnet
```

### Transferring Tokens

1. **Transferring Tokens to a Friend**
   I transferred some tokens to my friend's wallet using:

```bash
spl-token transfer <token-address> 150 <friend-wallet-address> --url devnet
```

I made sure to use `--fund-recipient` to fund the recipient's account if necessary.
