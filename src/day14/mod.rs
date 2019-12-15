use std::collections::HashMap;
use super::utils::ParseError;

#[derive(Debug)]
struct Reagent<'a> {
  amount: u32,
  what: &'a str,
}

impl<'a> Reagent<'a> {
  fn new(amount: u32, what: &'a str) -> Reagent<'a> {
    Reagent { amount, what }
  }

  fn from_str(s: &'a str) -> Result<Reagent<'a>, ParseError> {
    let mut data = s.split(" ");
    let amount_str = data.next();
    let what_str = data.next();

    if let (Some(amount), Some(what)) = (amount_str, what_str) {
      let amount = amount.parse::<u32>()?;
      Ok(Reagent::new(amount, what))
    } else {
      Err(ParseError::new("Could not parse reaction agent"))
    }
  }
}

#[derive(Debug)]
struct Reaction<'a> {
  input: Vec<Reagent<'a>>,
  output: Reagent<'a>,
}

impl<'a> Reaction<'a> {
  fn new((input, output): (Vec<Reagent<'a>>, Reagent<'a>)) -> Reaction<'a> {
    Reaction { input, output }
  }
}

fn parse_input(input: &str) -> Result<HashMap<&str, Vec<&str>>, ParseError> {
  let reactions = input
    .lines()
    .filter(|v| *v != "")
    .map(|v| v.split("=>").collect::<Vec<_>>())
    .map(|v| (v[0].split(",").map(|v| Reagent::from_str(v.trim())).collect::<Result<Vec<_>, ParseError>>(), Reagent::from_str(v[1].trim())))
    .map(|v| {
      if let (Ok(input), Ok(output)) = v {
        Ok(Reaction::new((input, output)))
      } else {
        Err(ParseError::new("Could not parse reaction"))
      }
    })
    .collect::<Result<Vec<_>, ParseError>>()?;

  println!("{:?}", reactions);
  Ok(HashMap::new())
}

pub fn problem1() -> Result<(), ParseError> {
  let input = include_str!("./data/input-1.txt");
  let reactions = parse_input(&input)?;
  Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
  Ok(())
}

#[cfg(test)]
mod test {

  #[test]
  fn problem1_example1() {

  }
}