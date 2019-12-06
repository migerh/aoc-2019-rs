use std::collections::HashMap;

fn count_orbits_for_body(map: &HashMap<&str, &str>, entry: &str) -> u32 {
  let mut current_body = entry;
  let mut count = 1;

  while current_body != "COM" {
    count += 1;
    current_body = map[current_body];
  }

  count
}

fn create_map(inputs: &str) -> HashMap<&str, &str> {
  let rules = inputs
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| v.split(")").collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut map = HashMap::new();
  for rule in rules {
    map.entry(rule[1]).or_insert(rule[0]);
  }

  map
}

fn count_orbits(input: &str) -> u32 {
  let map = create_map(input);
  // traverse the tree
  let mut sum = 0;
  for (_k, v) in map.iter() {
    sum += count_orbits_for_body(&map, v);
  }

  sum
}

pub fn problem1() -> u32 {
  let input = include_str!("./data/input-1.txt");
  let result = count_orbits(input);

  println!("result: {}", result);

  result
}

fn traverse<'a>(map: &HashMap<&'a str, &'a str>, entry: &'a str) -> Vec<&'a str> {
  let mut current = entry;
  let mut result = vec![];
  while current != "COM" {
    result.push(current);
    current = map[current];
  }

  result
}

fn find_closest_common_body<'a>(map: &HashMap<&'a str, &'a str>, body1: &'a str, body2: &'a str) -> &'a str {
  let path_body1 = traverse(map, body1);
  let path_body2 = traverse(map, body2);

  let mut first_common = "COM";
  for body in path_body1 {
    if path_body2.contains(&body) {
      first_common = body;
      break;
    }
  }

  first_common
}

fn calculate_shortest_path_length(input: &str) -> u32 {
  let map = create_map(input);
  let orbits_for_me = count_orbits_for_body(&map, "YOU");
  let orbits_for_santa = count_orbits_for_body(&map, "SAN");

  let closest_common_body = find_closest_common_body(&map, "YOU", "SAN");
  let orbits_for_common_body = count_orbits_for_body(&map, closest_common_body);

  orbits_for_me - orbits_for_common_body + orbits_for_santa - orbits_for_common_body - 2
}

pub fn problem2() -> u32 {
  let input = include_str!("./data/input-1.txt");
  let result = calculate_shortest_path_length(input);

  println!("result: {}", result);
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example() {
    let input = include_str!("./data/example.txt");
    assert_eq!(count_orbits(input), 42);
  }

  #[test]
  fn problem2_example() {
    let input = include_str!("./data/example2.txt");
    assert_eq!(calculate_shortest_path_length(input), 4);
  }
}