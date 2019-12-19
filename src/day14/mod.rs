use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;
use super::utils::ParseError;

#[derive(Debug)]
struct Reagent<'a> {
  amount: u64,
  what: &'a str,
}

impl<'a> Reagent<'a> {
  fn new(amount: u64, what: &'a str) -> Reagent<'a> {
    Reagent { amount, what }
  }

  fn from_str(s: &'a str) -> Result<Reagent<'a>, ParseError> {
    let mut data = s.split(" ");
    let amount_str = data.next();
    let what_str = data.next();

    if let (Some(amount), Some(what)) = (amount_str, what_str) {
      let amount = amount.parse::<u64>()?;
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

#[derive(Debug)]
struct Lab<'a> {
  reactions: RefCell<Vec<Reaction<'a>>>,
  shelf: RefCell<HashMap<&'a str, u64>>,
  used_materials: RefCell<HashMap<&'a str, u64>>,
  debug: bool,
}

impl<'a> Lab<'a> {
  fn new(reactions: Vec<Reaction<'a>>) -> Lab<'a> {
    let shelf = RefCell::new(HashMap::new());
    let used_materials = RefCell::new(HashMap::new());
    let reactions = RefCell::new(reactions);
    let debug = false;

    Lab { reactions, shelf, used_materials, debug }
  }

  fn clear(&self) {
    self.shelf.borrow_mut().clear();
    self.used_materials.borrow_mut().clear();
  }

  fn log(&self, msg: String) {
    if self.debug {
      println!("{}", msg);
    }
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

  fn produce(&'a self, what: &'a str, amount: u64) {
    let mut backlog = VecDeque::new();
    backlog.push_back((what, amount));

    while let Some(next) = backlog.pop_front() {
      let (next_what, next_amount) = next;
      if next_what == "ORE" {
        *self.used_materials.borrow_mut().entry("ORE").or_insert(0) += next_amount;
        continue;
      }

      let relevant_reactions = self.find_reactions_with_output(next_what);

      if relevant_reactions.is_empty() {
        panic!("Don't know how to produce {}", next_what);
      }

      if relevant_reactions.len() > 1 {
        panic!("Too many reactions ({}) to produce {}", relevant_reactions.len(), next_what);
      }


      let reactions = self.reactions.borrow();
      let reaction = reactions.get(relevant_reactions[0]).unwrap();
      let oa = reaction.output.amount;

      let excess = if let Some(shelved) = self.shelf.borrow().get(next_what) {
        *shelved
      } else {
        0
      };

      if excess > next_amount {
        *self.shelf.borrow_mut().entry(next_what).or_insert(0) = excess - next_amount;
        continue;
      }
      let factor = (next_amount - excess + (oa - 1)) / oa;

      let excess = oa * factor - (next_amount - excess);
      self.log(format!("Used {} {}", next_amount, next_what));
      *self.used_materials.borrow_mut().entry(next_what).or_insert(0) += next_amount;
      self.log(format!("Shelving {} {}", excess, next_what));
      *self.shelf.borrow_mut().entry(next_what).or_insert(0) = excess;

      for reagent in &reaction.input {
        self.log(format!("Producing {} {}", reagent.amount * factor, reagent.what));
        backlog.push_back((reagent.what, factor * reagent.amount));
      }

      self.log(format!("Backlog: {:?}", backlog));
    }
  }
}

pub fn problem1() -> Result<u64, ParseError> {
  let input = include_str!("./data/input-1.txt");
  // let input = include_str!("./data/example-4.txt");
  let lab = Lab::parse(&input)?;

  lab.produce("FUEL", 1);
  if let Some(result) = lab.used_materials.borrow().get("ORE") {
    println!("Amount of ORE used for 1 FUEL: {}", result);
    return Ok(*result);
  }

  Err(ParseError::new("Could not find ORE"))
}

pub fn problem2() -> Result<u64, ParseError> {
  let input = include_str!("./data/input-1.txt");
  let lab = Lab::parse(&input)?;

  let ore = 1_000_000_000_000u64;
  let mut high = 1_000_000_000_000u64;
  let mut low = 1u64;
  loop {
    if high - low <= 1 {
      break;
    }
    let middle = low + (high - low) / 2;

    lab.clear();
    lab.produce("FUEL", middle);

    if let Some(result) = lab.used_materials.borrow().get("ORE") {
      println!("Looking at {} we need {} ore", middle, result);
      if *result < ore {
        low = middle;
      } else {
        high = middle;
      }
    } else {
      return Err(ParseError::new("Could not find ORE"));
    }
  }

  let result = low;
  println!("Result 14-2: {}", result);
  Ok(result)
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
    assert_eq!(lab.used_materials.borrow().get("ORE").unwrap(), &13312);

    Ok(())
  }

  #[test]
  fn problem1_example4() -> Result<(), ParseError> {
    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
    let lab = Lab::parse(&input)?;
    lab.produce("FUEL", 1);
    assert_eq!(lab.used_materials.borrow().get("ORE").unwrap(), &180697);

    Ok(())
  }

  #[test]
  fn problem1_example5() -> Result<(), ParseError> {
    let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
    let lab = Lab::parse(&input)?;
    lab.produce("FUEL", 1);
    assert_eq!(lab.used_materials.borrow().get("ORE").unwrap(), &2210736);

    Ok(())
  }

}