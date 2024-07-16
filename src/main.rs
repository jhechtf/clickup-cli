mod cli;
mod readline;
mod server;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), String> {
  dotenv().ok();

  let mut runner = cli::cli_util::CliRunner::init();
  runner.run().await?;
  Ok(())
}
