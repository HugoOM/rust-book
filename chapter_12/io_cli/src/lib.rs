use std::env;
use std::{error::Error, fs};

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: env::Args) -> Result<Config, &'static str> {
    args.next();

    if args.len() != 3 {
      return Err("Incorrect arguments provided.");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents =
    fs::read_to_string(config.filename).expect("Something went wrong reading the file");

  for line in search(&config.query, &contents) {
    println!("{}", line)
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}
