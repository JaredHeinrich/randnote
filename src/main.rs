use std::process::exit;

use anyhow::Result;
use app::App;
use clap::Parser;

use crate::{cli::Cli, file_operations::FileSystem, message::Message};

mod app;
mod cli;
mod config;
mod error;
mod file_operations;
mod message;

#[cfg(test)]
mod mock_fs;

fn main() {
    let result = run();
    print_result(&result);
    if result.is_err() {
        exit(1);
    }
}

fn run() -> Result<Message> {
    let fs = FileSystem;
    let config = config::Config::build(&fs)?;
    let mut app = App::new(config, fs)?;
    let command = Cli::parse();
    app.handle_command(command)
}

#[allow(clippy::print_stdout)] // global output of cli tool
fn print_result(result: &Result<Message>) {
    match result {
        Ok(m) => print!("{m}"),
        Err(e) => print!("{e}"),
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
