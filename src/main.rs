mod cli;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    cli::run(Cli::parse())
}
