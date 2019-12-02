#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;
mod utils;

fn run() -> Result<(), utils::Error> {
  day2::problem1();
  day2::problem2();

  if false {
    day1::problem1();
    day1::problem2();
  }
  Ok(())
}

fn main() {
  match run() {
    Err(err) => println!("Error occurred: {}", err),
    _ => {}
  }
}
