#[path = "src/cli.rs"]
mod cli;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
};
use cli::Cli;
use std::path::Path;
use std::{fs::create_dir, io::Result};

fn main() -> Result<()> {
    let cmd = &mut Cli::cmd();
    let name = "nml";
    let dir = "completions";
    if !Path::new(dir).exists() {
        create_dir(dir)?;
    }
    generate_to(Bash, cmd, name, dir)?;
    generate_to(Zsh, cmd, name, dir)?;
    generate_to(Fish, cmd, name, dir)?;
    Ok(())
}
