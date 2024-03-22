use clap::{ Command, arg };
use std::process::exit;
use bitcoincash_addr::{Address};

use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::wallet::Wallets;
use crate::transaction::Transaction;
use crate::utxoset::UTXOSet;

pub struct Cli {
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
        })
    }
    
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("xuerong@nanopay.net")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(
                Command::new("print-chain")
                    .about("print all the chain blocks.")
            )
            .subcommand(
                Command::new("get-balance")
                    .about("get balance of a address in the blockchain.")
                    .arg(arg!(<ADDRESS>"'The address in blockchain.'"))
            )
            .subcommand(
                Command::new("create")
                    .about("Create a new blockchain.")
                    .arg(arg!(<ADDRESS>"'The address to send genesis block reward to'"))
            )
            .subcommand(
                Command::new("send")
                    .about("Send fund in the blockchain.")
                    .arg(arg!(<FROM>" 'Source wallet address'"))
                    .arg(arg!(<TO> " 'Destination wallet address'"))
                    .arg(arg!(<AMOUNT> " 'Amount to send'"))
            )
            .subcommand(
                Command::new("create-wallet")
                    .about("create a wallet")
            )
            .subcommand(
                Command::new("list-addresses")
                    .about("list all addresses")
            )
            .subcommand(
                Command::new("reindex")
                    .about("Re-index the UTXOSet")
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let blockchain = Blockchain::create_blockchain(String::from(address))?;
                let utxo_set = UTXOSet::new(blockchain);
                utxo_set.reindex()?;
                println!("create blockchain");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("get-balance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let pub_key_hash = Address::decode(address).unwrap().body;
                let bc = Blockchain::new()?;
                let utxo_set = UTXOSet::new(bc);
                let utxos = utxo_set.find_UTXO(&pub_key_hash)?;

                let mut balance = 0;
                for out in utxos.outputs {
                    balance += out.value;
                }
                println!("Balance of `{}`; {}", address, balance)
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("`from` not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("`to` not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("`amount` no supply!: usage");
                exit(1)
            };

            let bc = Blockchain::new()?;
            let mut utxo_set = UTXOSet::new(bc);
            let tx = Transaction::new_UTXO(from, to, amount, &utxo_set)?;
            let cbtx = Transaction::new_coinbase(from.to_string(), String::from("reward!"))?;
            let new_block = utxo_set.blockchain.add_block(vec![cbtx, tx])?;

            utxo_set.update(&new_block)?;
            println!("success!");
        }

        if let Some(_) = matches.subcommand_matches("reindex") {
            let bc = Blockchain::new()?;
            let utxo_set = UTXOSet::new(bc);
            utxo_set.reindex()?;
            let count = utxo_set.count_transactions()?;
            println!("Done! There are {count} transactions in the UTXO set.");
        }

        if let Some(_) = matches.subcommand_matches("print-chain") {
            let bc = Blockchain::new()?;
            for b in bc.iter() {
                println!("Block: {:#?}", b);
            }
        }

        if let Some(_) = matches.subcommand_matches("create-wallet") {
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("Wallet create successed with address `{}`", address);
        }

        if let Some(_) = matches.subcommand_matches("list-addresses") {
            let ws = Wallets::new()?;
            let addresses = ws.get_all_address();
            println!("addresses: ");
            for ad in addresses {
                println!("{}", ad);
            }
        }

        Ok(())
    }
}