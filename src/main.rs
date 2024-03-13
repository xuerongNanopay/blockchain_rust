mod block;
mod errors;
mod blockchain;
mod cli;

use errors::Result;
use cli::Cli;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    
    Ok(())
}
