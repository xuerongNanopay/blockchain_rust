use crate::blockchain::Blockchain;
use crate::errors::Result;
use clap::{ Command, arg };

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
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("Address") {
                let address = String::from(address);
                Blockchain::create_blockchain(address)?;
                println!("create blockchain");
            }
        }

        if let Some(_)  = matches.subcommand_matches("print-chain") {
            // self.print_chain();
        }

        Ok(())
    }

    // fn print_chain(&self) {
    //     for b in self.bc.iter() {
    //         println!("Block: {:#?}", b);
    //     }
    // }
}