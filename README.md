# blockchain_rust
Blockchain Programming from scratch with Rust

Wallet:
  - A paire of private and public key
  - Each wallet has its own address which generate/hash from public key.(see get_address)

Transaction:
  - id, a list of TXInput and a list of TxOutput.
  - id: hash if transactiont itself.
  - TXInput: specify where the fund come from. Refer to other TXOutput in previous transaction.
  - TXOutput: sepcify the amount move into associated address. Would comsume by future TXInput
  - Signiture: What should incluld in the signiture.
    - 

Thinks:
  - Why do we need to hash the public key?