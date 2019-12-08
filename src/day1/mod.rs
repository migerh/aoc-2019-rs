fn fuel_for_one(mass: i32) -> i32 {
  mass / 3 - 2
}

fn fuel_iterative(mass: i32) -> i32 {
  let mut fuel = fuel_for_one(mass);
  let mut total_fuel = fuel;

  while fuel > 0 {
    fuel = fuel_for_one(fuel);
    if fuel > 0 {
      total_fuel += fuel;
    }
  }

  total_fuel
}

fn fuel_for_delivery(f: &dyn Fn(i32) -> i32) -> i32 {
  let input = include_str!("./data/input.txt");
  let total_fuel: i32 = input
    .lines()
    .filter(|v| *v != "")
    .map(|v| v.parse::<i32>().unwrap())
    .map(|m| f(m))
    .sum();

  total_fuel
}

pub fn problem1() {
  println!("Fuel sum: {}", fuel_for_delivery(&fuel_for_one));
}

pub fn problem2() {
  println!("Fuel sum iterative: {}", fuel_for_delivery(&fuel_iterative));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn iterative_fuel_example_1() {
    assert_eq!(fuel_iterative(1969), 966);
  }

  #[test]
  fn iterative_fuel_example_2() {
    assert_eq!(fuel_iterative(100756), 50346);
  }
}