use std::collections::HashMap;
use std::cell::RefCell;
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

  fn has_ore_input(&self) -> bool {
    for reagent in &self.input {
      if reagent.what == "ORE" {
        return true;
      }
    }

    false
  }

  fn amount_of_ore_input(&self) -> u32 {
    if !self.has_ore_input() {
      return 0;
    }

    for reagent in &self.input {
      if reagent.what == "ORE" {
        return reagent.amount;
      }
    }

    0
  }
}

#[derive(Debug)]
struct Lab<'a> {
  reactions: RefCell<Vec<Reaction<'a>>>,
  shelf: RefCell<HashMap<&'a str, u32>>,
  used_materials: RefCell<HashMap<&'a str, u32>>,
}

impl<'a> Lab<'a> {
  fn new(reactions: Vec<Reaction<'a>>) -> Lab<'a> {
    let shelf = RefCell::new(HashMap::new());
    let used_materials = RefCell::new(HashMap::new());
    let reactions = RefCell::new(reactions);

    Lab { reactions, shelf, used_materials }
  }

  fn parse(input: &str) -> Result<Lab, ParseError> {
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

      Ok(Lab::new(reactions))
  }

  fn find_reactions_with_output(&self, output: &str) -> Vec<usize> {
    let mut result = vec![];

    for (index, reaction) in self.reactions.borrow().iter().enumerate() {
      if reaction.output.what == output {
        result.push(index);
      }
    }

    result
  }

  fn produce(&'a self, what: &'a str, amount: u32) {
    if what == "ORE" {
      *self.used_materials.borrow_mut().entry("ORE").or_insert(0) += amount;
      return;
    }

    let relevant_reactions = self.find_reactions_with_output(what);

    if relevant_reactions.is_empty() {
      panic!("Don't know how to produce {}", what);
    }

    if relevant_reactions.len() > 1 {
      panic!("Too many reactions ({}) to produce {}", relevant_reactions.len(), what);
    }

    let reactions = self.reactions.borrow();
    let reaction = reactions.get(relevant_reactions[0]).unwrap();
    let oa = reaction.output.amount;
    let factor = (amount + (oa - 1)) / oa;
    let excess = oa * factor - amount;
    println!("Used {} {}", amount, what);
    *self.used_materials.borrow_mut().entry(what).or_insert(0) += amount;
    println!("Shelving {} {}", excess, what);
    *self.shelf.borrow_mut().entry(what).or_insert(0) += excess;

    for reagent in &reaction.input {
      if let Some(shelved) = self.shelf.borrow_mut().get_mut(reagent.what) {
        if *shelved > factor * reagent.amount {
          println!("Reusing {} units of shelved {}", factor * reagent.amount, reagent.what);
          *shelved -= factor * reagent.amount;
          continue;
        }
      }

      println!("Producing {} {}", reagent.amount * factor, reagent.what);
      self.produce(reagent.what, factor * reagent.amount);
    }
  }

  fn optimize(&self) {
    for (key, value) in self.shelf.borrow_mut().iter() {
      let reactions = self.reactions.borrow();
      let relevant_reaction = reactions.get(self.find_reactions_with_output(*key)[0]).unwrap();
      let oa = relevant_reaction.output.amount;
      if relevant_reaction.has_ore_input() && oa < *value {
        let factor = *value / oa;
        let ore_input = relevant_reaction.amount_of_ore_input();
        *self.used_materials.borrow_mut().get_mut("ORE").unwrap() -= factor * ore_input;
      }
    }
  }

  fn get_produced(&self) -> HashMap<&str, u32> {
    let mut map = HashMap::new();

    for reagent in self.reactions.borrow().iter() {
      let what = reagent.output.what;
      let entry = map.entry(what).or_insert(0);
      *entry += self.used_materials.borrow().get(what).unwrap();
      *entry += self.shelf.borrow().get(what).unwrap();
    }

    map
  }
}

pub fn problem1() -> Result<u32, ParseError> {
  let input = include_str!("./data/input-1.txt");
  let lab = Lab::parse(&input)?;

  println!("reactions: {:?}", lab);
  lab.produce("FUEL", 1);
  if let Some(result) = lab.used_materials.borrow().get("ORE") {
    println!("Amount of ORE used for 1 FUEL: {}", result);
    return Ok(*result);
  }

  Err(ParseError::new("Could not find ORE"))

  // let fuel_reactions = find_fuel_reactions(&reactions);
  // println!("Fuel reactions ({:?})", fuel_reactions);

  // for index in fuel_reactions {
  //   println!("reactions {:?}", reactions[index]);
  // }

  // let mut map = HashMap::new();
  // for (index, reaction) in reactions.iter().enumerate() {
  //   map.entry(reaction.output.what)
  //     .or_insert(vec![]).push(index);
  // }

  // println!("Multiplicity check");
  // for (key, value) in map.iter() {
  //   if value.len() > 1 {
  //     println!("More than one reaction to create {}", key);
  //   }
  // }

  // println!("{:?}", map);
}

pub fn problem2() -> Result<(), ParseError> {
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example1() -> Result<(), ParseError> {
    let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
    let lab = Lab::parse(&input)?;
    lab.produce("FUEL", 1);
    lab.optimize();
    assert_eq!(lab.used_materials.borrow().get("ORE").unwrap(), &31);

    Ok(())
  }

  #[test]
  fn problem1_example2() -> Result<(), ParseError> {
    let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
    let lab = Lab::parse(&input)?;
    lab.produce("FUEL", 1);
    lab.optimize();
    let produced = lab.get_produced();
    println!("Produced: {:?}", produced);
    println!("Shelf: {:?}", lab.shelf.borrow());
    println!("Consumed: {:?}", lab.used_materials.borrow());

    assert_eq!(lab.used_materials.borrow().get("ORE").unwrap(), &165);
    Ok(())
  }

  #[test]
  fn problem1_example3() -> Result<(), ParseError> {
    let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    let lab = Lab::parse(&input)?;
    lab.produce("FUEL", 1);
    lab.optimize();
    // assert_eq!(lab.used_materials.borrow().get("ORE").unwrap(), &13311);

    Ok(())
  }
}