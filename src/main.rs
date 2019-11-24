#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day1;
mod utils;

fn run() -> Result<(), utils::Error> {
  day1::problem1();
  Ok(())
}

fn main() {
  match run() {
    Err(err) => println!("Error occurred: {}", err),
    _ => {}
  }
}
