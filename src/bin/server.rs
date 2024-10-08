use std::{
  io::{Read, Write},
  net::{TcpListener, TcpStream},
};

use anyhow::{Context, Ok};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  port: usize,
}

fn build_tcp_string(port: usize) -> String {
  format!("127.0.0.1:{}", port)
}

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
  let mut client_sent_file = String::new();
  stream
    .read_to_string(&mut client_sent_file)
    .context("Failed to read file sent from client")?;

  println!("{client_sent_file}");

  let alphabetic_count = client_sent_file
    .chars()
    .filter(|c| c.is_alphabetic())
    .count();

  let alphabetic_count_string = alphabetic_count.to_string();

  stream
    .write(alphabetic_count_string.as_bytes())
    .context("Failed to write to client")?;

  Ok(())
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();
  let tcp_string = build_tcp_string(args.port);
  let listener = TcpListener::bind(tcp_string).context("Failed to create TCP Server.")?;

  for stream in listener.incoming() {
    handle_client(stream.context("Client failed to connect for unkown reason.")?)?;
  }

  Ok(())
}
