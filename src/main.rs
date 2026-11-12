use std::process::exit;

use clap::Parser;
use randnote::{
    cli::Cli,
    config,
    core::{self, error::Result},
    message::Message,
};

fn main() {
    let result = run();
    print_result(&result);
    if result.is_err() {
        exit(1);
    }
}

fn run() -> Result<Message> {
    let config = config::Config::build()?;
    let command = Cli::parse();
    core::handle_command(&config, command)
}

#[allow(clippy::print_stdout)] // global output of cli tool
fn print_result(result: &Result<Message>) {
    match result {
        Ok(m) => println!("{m}"),
        Err(e) => println!("{e}"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn check_zsh_completion_reference_consistency() {
        let generated = include_str!(concat!(env!("OUT_DIR"), "/_rn"));
        let checked_in = include_str!("../completions/zsh.reference");
        assert!(
            generated == checked_in,
            "completions/zsh.reference outdated"
        );
    }
}
