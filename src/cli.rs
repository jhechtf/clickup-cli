pub mod cli_util;
pub mod login;

use clap::{CommandFactory, Subcommand};

#[derive(clap::Parser, Clone, Debug)]
enum Command {
  // #[command(subcommand)]
  // List(ListCommand),
  Login,
}

#[derive(clap::Subcommand, Clone, Debug)]
enum ListCommand {
  All,
}

#[derive(clap::Parser, Clone, Debug)]
#[command(name = "cup")]
pub struct Args {}

pub fn default_app() -> clap::Command {
  Command::augment_subcommands(Args::command())
}

#[derive(Debug)]
pub enum CommandResult {
  Loop,
  Run,
}
