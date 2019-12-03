use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
enum Direction {
  Right(u32),
  Up(u32),
  Left(u32),
  Down(u32)
}

impl FromStr for Direction {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static!{
      static ref RE: Regex = Regex::new(r"([RULD])(\d+)").unwrap();
    }
    let cap = RE.captures(s).unwrap();
    let steps = cap[2].parse::<u32>()?;
    Ok(match &cap[1] {
      "R" => Direction::Right(steps),
      "U" => Direction::Up(steps),
      "L" => Direction::Left(steps),
      "D" => Direction::Down(steps),
      _ => panic!("No parse, replace me with proper error handling")
    })
  }
}

fn parse_single_wire(input: &str) -> Result<Vec<Direction>, ParseError> {
  input
    .split(",")
    .filter(|v| *v != "")
    .map(|v| Direction::from_str(v))
    .collect::<Result<Vec<Direction>, ParseError>>()
}

fn parse_input(input: &str) -> Result<Vec<Vec<Direction>>, ParseError> {
  input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| parse_single_wire(v))
    .collect::<Result<Vec<Vec<Direction>>, ParseError>>()
}

fn trace_wire(map: &mut HashMap<(i32, i32), (HashSet<usize>, Vec<(usize, usize)>)>, wire: &Vec<Direction>, index: usize) {
  let mut last_corner = (0, 0);
  let mut steps = 0;

  for step in wire {
    let m: (i32, i32, u32) = match step {
      Direction::Right(n) => (1, 0, *n),
      Direction::Up(n) => (0, 1, *n),
      Direction::Left(n) => (-1, 0, *n),
      Direction::Down(n) => (0, -1, *n)
    };

    let end = m.2 as usize;
    for i in 1..end+1 {
      let coords = (last_corner.0 + (i as i32) * m.0, last_corner.1 + (i as i32) * m.1);
      steps += 1;
      let entry = map
        .entry(coords)
        .or_insert((HashSet::new(), vec![]));
      entry.0.insert(index);
      entry.1.push((index, steps));
    }

    last_corner.0 += (m.2 as i32) * m.0;
    last_corner.1 += (m.2 as i32) * m.1;
  }
}

pub fn run_problem1(input: &str) -> Result<i32, ParseError> {
  let wires = parse_input(input)?;
  let mut map: HashMap<(i32, i32), (HashSet<usize>, Vec<(usize, usize)>)> = HashMap::new();

  for (i, wire) in wires.iter().enumerate() {
    trace_wire(&mut map, &wire, i);
  }

  let mut intersections = vec![];
  for (k, v) in map.iter() {
    if v.0.len() > 1 {
      if k.0 != 0 || k.1 != 0 {
        intersections.push(k);
      }
    }
  }

  let mut distances = intersections
    .iter()
    .map(|&v| v.0.abs() + v.1.abs())
    .collect::<Vec<i32>>();

  distances.sort();
  let smallest = distances[0];
  Ok(smallest)
}

fn get_sorted_distances_for_wire_i(distances: &Vec<(usize, usize)>, i: usize) -> Vec<usize> {
  let mut distances_for_wire_i = distances
    .iter()
    .filter(|&v| v.0 == i)
    .map(|&v| v.1)
    .collect::<Vec<_>>();
  distances_for_wire_i.sort();

  distances_for_wire_i
}

fn get_shortest_distance(distances: &Vec<(usize, usize)>) -> usize {
  let shortest_walking_distance_for_wire_0 = get_sorted_distances_for_wire_i(distances, 0)[0];
  let shortest_walking_distance_for_wire_1 = get_sorted_distances_for_wire_i(distances, 1)[0];

  shortest_walking_distance_for_wire_0 + shortest_walking_distance_for_wire_1
}

fn run_problem2(input: &str) -> Result<usize, ParseError> {
  let wires = parse_input(input)?;
  let mut map: HashMap<(i32, i32), (HashSet<usize>, Vec<(usize, usize)>)> = HashMap::new();

  for (i, wire) in wires.iter().enumerate() {
    trace_wire(&mut map, &wire, i);
  }

  let mut intersections = vec![];
  for (k, v) in map.iter() {
    if v.0.len() > 1 {
      if k.0 != 0 || k.1 != 0 {
        intersections.push(v.1.clone());
      }
    }
  }

  let mut distances = intersections
    .iter()
    .map(|v| get_shortest_distance(v))
    .collect::<Vec<_>>();

  distances.sort();

  Ok(distances[0])
}

pub fn problem1() -> Result<i32, ParseError> {
  let input = include_str!("./data/input-1.txt");
  let result = run_problem1(input)?;

  println!("Result: {}", result);

  Ok(result)
}

pub fn problem2() -> Result<usize, ParseError> {
  let input = include_str!("./data/input-1.txt");
  let result = run_problem2(input)?;

  println!("Result: {}", result);

  Ok(result)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example1() {
    assert_eq!(run_problem1("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap(), 6);
    assert_eq!(run_problem1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").unwrap(), 159);
    assert_eq!(run_problem1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap(), 135);
  }

  #[test]
  fn problem2_example1() {
    assert_eq!(run_problem2("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap(), 30);
    assert_eq!(run_problem2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").unwrap(), 610);
    assert_eq!(run_problem2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap(), 410);
  }
}