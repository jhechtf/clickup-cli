// use std::{
//   io::{prelude::*, BufReader},
//   net::TcpListener,
// };

use rouille::{Request, Response, Server};

pub(crate) fn start_server(port: u16) {
  let server_string = format!("127.0.0.1:{port}");

  let server = Server::new(server_string, move |request| {
    let code = request.raw_query_string();
    println!("code: {code}");
    Response::text("Hello world!")
  })
  .unwrap();

  loop {
    server.poll()
  }

  // rouille::start_server("127.0.0.1:5999", move |request| {
  //   let code = request.get_param("code").unwrap();
  //   println!("code: {code}");
  //   Response::text("Hello world!")
  // });
  // let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

  // let mut count = 0;

  // for stream in listener.incoming() {
  //   let stream = stream.unwrap();
  //   let mut stream = stream;
  //   let buf_reader = BufReader::new(&mut stream);
  //   let http_request: Vec<_> = buf_reader
  //     .lines()
  //     .map(|result| result.unwrap())
  //     .take_while(|line| !line.is_empty())
  //     .collect();

  //   println!("Request: {http_request:#?}");

  //   let response = "HTTP/1.1 200 OK\r\n\r\n";

  //   stream.write_all(response.as_bytes()).unwrap();
  //   count = count + 1;
  //   println!("New connection: {stream:?}");

  //   if count == 10 {
  //     break;
  //   }
  // }
}
