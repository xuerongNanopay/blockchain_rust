use crate::blockchain::Blockchain;
use crate::errors::Result;
use clap::{ Command, arg };

pub struct Cli {
    bc: Blockchain
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }
    
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("behorouz.r.fa@gmail.com")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(Command::new("printChain").about("print all the chain blocks"))
            .subcommand(
                Command::new("addBlock")
                .about("add a block in the blockchain")
                .arg(arg!(<DATA>" 'the blockchain data'")),
            )
            .get_matches();
        
        if let Some(ref matches) = matches.subcommand_matches("addBlock") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.add_block(String::from(c))?;
            } else {
                println!("Not printing testing lists...");
            }
        }

        if let Some(_)  = matches.subcommand_matches("printChain") {
            self.print_chain();
        }

        Ok(())
    }

    fn add_block(&mut self, data: String) -> Result<()> {
        self.bc.add_block(data)
    }

    fn print_chain(&self) {
        for b in self.bc.iter() {
            println!("Block: {:#?}", b);
        }
    }
}