use anyhow::{anyhow, Result};
use clap::CommandFactory;
use clap_complete::aot::Zsh;
use clap_complete::generate_to;
use std::env;

include!("src/cli/mod.rs");

fn main() -> Result<()> {
    let Some(outdir) = env::var_os("OUT_DIR") else {
        return Err(anyhow!("Environment Variable `OUT_DIR` not found"));
    };
    let mut cmd = crate::Cli::command();
    let path = generate_to(Zsh, &mut cmd, "rn", &outdir)?;
    println!(
        "cargo:warning=zsh completion file is generated: {}",
        path.display()
    );
    Ok(())
}
