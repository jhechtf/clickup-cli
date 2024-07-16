use crate::server::start_server;

pub(crate) async fn cmd_login() {
  println!("1. Start a server");
  let client_id = std::env::var("CLICKUP_CLIENT").expect("CLICKUP_CLIENT must be set.");
  let redirect_uri = "http://localhost:5999";
  println!("https://app.clickup.com/api?client_id={client_id}&redirect_uri={redirect_uri}");
  start_server(5999);
  println!("2. Open a browser to the login URL");
  println!("3. Wait for the logint to complete and we get the token value back");
  println!("4. Store the token value in the config file");
}
