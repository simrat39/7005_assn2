use std::{
  io::{Read, Write},
  net::TcpStream,
  path::Path,
  process::exit,
};

use anyhow::Context;
use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  server_ip: String,
  #[arg(short, long)]
  port: usize,
  #[arg(short, long)]
  file: String,
}

fn does_file_exist(file_path: &str) -> bool {
  let path = Path::new(file_path);
  return path.exists() && path.is_file();
}

fn read_file(file_path: &str) -> anyhow::Result<String> {
  std::fs::read_to_string(file_path)
    .with_context(|| format!("Failed to read file at path {}", file_path))
}

fn build_tcp_string(addr: String, port: usize) -> String {
  format!("{}:{}", addr, port)
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let file_path = args.file;
  if !does_file_exist(&file_path) {
    eprintln!("File at path {file_path} doesn't exist. Exiting.");
    exit(1);
  }

  let file_contents = read_file(&file_path)?;

  let tcp_string = build_tcp_string(args.server_ip, args.port);
  let mut stream = TcpStream::connect(tcp_string).context("Failed to connect")?;

  stream
    .write(file_contents.as_bytes())
    .context("Failed to write to TCP stream.")?;

  stream
    .shutdown(std::net::Shutdown::Write)
    .context("Failed to shutdown write stream")?;

  let mut recv_buf = String::new();
  stream
    .read_to_string(&mut recv_buf)
    .context("Failed to read from TCP stream")?;

  println!("Number of alphabetic letters: {recv_buf}");

  Ok(())
}
