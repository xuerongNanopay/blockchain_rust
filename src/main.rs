mod block;
mod errors;
mod blockchain;
mod cli;
mod transaction;
mod tx;
mod wallet;

use errors::Result;
use cli::Cli;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
