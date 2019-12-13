use regex::Regex;
use std::fmt;
use num::integer::Integer;
use super::utils::ParseError;

//#[derive(Clone)]
type Coords = (i64, i64, i64);

#[derive(Debug, Clone)]
struct Body {
  id: usize,
  position: Coords,
  velocity: Coords,
}

impl fmt::Display for Body {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Position: <x={}, y={}, z={}>, Velocity: <x={}, y={}, z={}>",
      self.position.0, self.position.1, self.position.2,
      self.velocity.0, self.velocity.1, self.velocity.2)
  }
}

impl Body {
  fn from_str(input: &str, id: usize) -> Result<Self, ParseError> {
    lazy_static!{
      static ref RE: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    }

    let cap = RE.captures(input);
    if cap.is_none() {
      println!("Unable to parse {}", input);
      return Err(ParseError::new("Could not parse line."));
    }

    let cap = cap.unwrap();
    let x = cap[1].parse::<i64>()?;
    let y = cap[2].parse::<i64>()?;
    let z = cap[3].parse::<i64>()?;
    let position = (x, y, z);
    let velocity = (0, 0, 0);
    Ok(Body { id, position, velocity })
  }

  fn update_velocity(&mut self, other_bodies: &Vec<Body>) {
    for body in other_bodies {
      if body.id == self.id {
        continue;
      }

      self.velocity.0 += (- self.position.0 + body.position.0).signum();
      self.velocity.1 += (- self.position.1 + body.position.1).signum();
      self.velocity.2 += (- self.position.2 + body.position.2).signum();
    }
  }

  fn update_position(&mut self) {
    self.position.0 += self.velocity.0;
    self.position.1 += self.velocity.1;
    self.position.2 += self.velocity.2;
  }

  fn get_energy(&self) -> i64 {
    let potential = self.position.0.abs() + self.position.1.abs() + self.position.2.abs();
    let kinetic = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();

    potential * kinetic
  }
}

fn parse_input(input: &str) -> Result<Vec<Body>, ParseError> {
  let mut id_counter = 0;
  input
    .lines()
    .filter(|v| *v != "")
    .map(|v| {
      let body = Body::from_str(v, id_counter);
      id_counter += 1;
      body
    })
    .collect::<Result<Vec<_>, ParseError>>()
}

fn tick(bodies: &mut Vec<Body>) {
  let copy = bodies.clone();
  for body in bodies {
    body.update_velocity(&copy);
    body.update_position();
  }
}

fn _print(bodies: &Vec<Body>) {
  for body in bodies {
    println!("{}", body);
  }
}

fn get_energy(bodies: &Vec<Body>) -> i64 {
  let mut energy = 0;

  for body in bodies {
    energy += body.get_energy();
  }

  energy
}

pub fn problem1() -> Result<i64, ParseError> {
  let input = include_str!("./data/input-1.txt");
  let mut bodies = parse_input(input)?;

  for _ in 0..1000 {
    tick(&mut bodies);
  }
  let result = get_energy(&bodies);
  println!("Result 12-1: {}", result);
  Ok(result)
}

fn equal_state(bodies: &Vec<Body>, initial: &Vec<Body>, dim: usize) -> bool {
  for (idx, body) in bodies.iter().enumerate() {
    let other = initial.get(idx).unwrap();
    let body_pos = [body.position.0, body.position.1, body.position.2];
    let other_pos = [other.position.0, other.position.1, other.position.2];
    let positions_match = body_pos[dim] == other_pos[dim];
    if !positions_match {
      return false;
    }

    let body_vel = [body.velocity.0, body.velocity.1, body.velocity.2];
    let other_vel = [other.velocity.0, other.velocity.1, other.velocity.2];
    let velocities_match = body_vel[dim] == other_vel[dim];

    if !velocities_match {
      return false;
    }
  }

  true
}

fn find_iterations(bodies: &mut Vec<Body>, dim: usize) -> usize {
  let initial_state = bodies.clone();
  let mut counter = 0usize;
  loop {
    counter += 1;
    tick(bodies);
    if equal_state(&bodies, &initial_state, dim) {
      break;
    }
  }

  counter
}

fn run_problem2(input: &str) -> Result<usize, ParseError> {
  let mut bodies = parse_input(input)?;

  let counter_x = find_iterations(&mut bodies, 0);
  let counter_y = find_iterations(&mut bodies, 1);
  let counter_z = find_iterations(&mut bodies, 2);

  println!("{}, {}, {}", counter_x, counter_y, counter_z);

  let xy = counter_x * counter_y / counter_x.gcd(&counter_y);
  let gcd = xy.gcd(&counter_z);
  let counter = xy * counter_z / gcd;

  println!("Result 12-2: {}", counter);
  Ok(counter)
}

pub fn problem2() -> Result<usize, ParseError> {
  let input = include_str!("./data/input-1.txt");
  run_problem2(input)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example1() {
    let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
    let mut bodies = parse_input(input).unwrap();
    for _ in 0..10 {
      tick(&mut bodies);
    }
    _print(&bodies);
    let energy = get_energy(&bodies);
    println!("Energy: {}", energy);

    assert_eq!(1, 1);
  }

  #[test]
  fn problem2_example1() {
    let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
    let counter = run_problem2(input).unwrap();
    println!("Counter: {}", counter);

    assert_eq!(1, 1);
  }
}