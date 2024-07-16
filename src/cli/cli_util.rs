use crate::{cli::Args, readline::readline};
use clap::{error::ErrorKind, ArgMatches, Command, CommandFactory, Error, FromArgMatches};
use std::{
  self,
  env::{self, ArgsOs},
  thread::sleep,
  time::Duration,
};

use super::{login::cmd_login, CommandResult};
pub(crate) struct CliRunner {
  app: Command,
  matches: ArgMatches,
  args: Args,
  mode: CommandResult,
}

#[derive(clap::Parser, Clone, Debug)]
pub enum CupCommand {
  Login,
}

impl CliRunner {
  pub fn init() -> Self {
    // Grab the cup command
    let cmd = CupCommand::command();
    // grab the string args
    let string_args = expand_args(env::args_os()).unwrap_or_else(|e| vec!["".to_string()]);
    // Set the mode based on the number of args
    let mode = if string_args.len() == 1 {
      CommandResult::Loop
    } else {
      CommandResult::Run
    };

    // Grab the matches and args
    let (matches, args) = match mode {
      CommandResult::Run => parse_args(&cmd, &string_args).unwrap(),
      CommandResult::Loop => (
        cmd
          .clone()
          // No help rn
          .disable_help_subcommand(true)
          // don't care about the binary name rn
          .no_binary_name(true)
          // allow external subcommands
          .allow_external_subcommands(true)
          .get_matches_from(vec![""]),
        Args {},
      ),
    };

    CliRunner {
      app: cmd,
      matches,
      args,
      mode,
    }
  }

  pub async fn run(&mut self) -> Result<(), String> {
    // If we are in loop mode, we should just loop forever
    match self.mode {
      CommandResult::Loop => loop {
        let line = readline()?;

        let vect = line.split_whitespace().collect::<Vec<&str>>();

        println!("{:?}", vect);

        self.matches = self.app.clone().no_binary_name(true).get_matches_from(vect);
        self.run_internal(&CupCommand::Login).await?;
      },
      CommandResult::Run => self.run_internal(&CupCommand::Login).await?,
    }

    Ok(())
  }

  async fn run_internal(&self, _subcommand: &CupCommand) -> Result<(), String> {
    let result = CupCommand::from_arg_matches(&self.matches)
      .map_err(|e| e.to_string())
      .unwrap();

    match result {
      CupCommand::Login => cmd_login().await,
      _ => unreachable!(),
    }
    Ok(())
  }

  pub fn args(&self) -> &Args {
    &self.args
  }
}

pub fn parse_args(app: &Command, string_args: &[String]) -> Result<(ArgMatches, Args), Error> {
  let matches = app
    .clone()
    .arg_required_else_help(true)
    .subcommand_required(true)
    .try_get_matches_from(string_args)?;

  let args = Args::from_arg_matches(&matches).unwrap();

  Ok((matches, args))
}

pub fn expand_args(args_os: ArgsOs) -> Result<Vec<String>, Error> {
  let mut string_args: Vec<String> = vec![];
  for arg_os in args_os {
    if let Some(string_arg) = arg_os.to_str() {
      string_args.push(string_arg.to_owned());
    }
  }

  Ok(string_args)
}
