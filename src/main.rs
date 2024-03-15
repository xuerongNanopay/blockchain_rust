mod block;
mod errors;
mod blockchain;
mod cli;
mod transaction;
mod tx;

use errors::Result;
use cli::Cli;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
