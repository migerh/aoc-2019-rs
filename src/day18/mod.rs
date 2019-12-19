use std::collections::{HashMap, HashSet};
use pathfinding::prelude::*;

type Coords = (i64, i64);

#[derive(Debug, PartialEq)]
enum Tile {
  Floor,
  Wall,
  Entry,
  Key(char),
  Door(char),
}

struct Vault {
  map: HashMap<Coords, Tile>,
  keys: HashSet<char>,
  collected_keys: HashSet<char>,
  entry: Coords,
}

impl Vault {
  fn new(input: &str) -> Vault {
    let mut map = HashMap::new();
    let mut keys = HashSet::new();
    let collected_keys = HashSet::new();
    let mut entry = (0, 0);

    let lines = input.lines();
    for (y, line) in lines.enumerate() {
      for (x, c) in line.chars().enumerate() {
        *map.entry((x as i64, y as i64)).or_insert(Tile::Wall) = match c {
          '#' => Tile::Wall,
          '.' => Tile::Floor,
          '@' => {
            entry = (x as i64, y as i64);
            Tile::Entry
          },
          v if Vault::is_door(v) => Tile::Door(c),
          v if Vault::is_key(v) => {
            keys.insert(v);
            Tile::Key(c)
          },
          _ => panic!("Unknown symbol: {}", c)
        };
      }
    }

    Vault { map, keys, entry, collected_keys }
  }

  fn is_door(c: char) -> bool {
    c.is_ascii_alphabetic() && c.is_ascii_uppercase()
  }

  fn is_key(c: char) -> bool {
    c.is_ascii_alphabetic() && c.is_ascii_lowercase()
  }

  fn shortest_path(&self, start: Coords, end: Coords) -> Option<(Vec<Coords>, i64)> {
    let map = &self.map;
    let collected_keys = &self.collected_keys;
    let neighbor = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    dijkstra(&start, |&(px, py)| {
      let mut next = Vec::new();

      for n in neighbor.iter() {
        let x = px + n.0;
        let y = py + n.1;
        if let Some(t) = map.get(&(x, y)) {
          if *t == Tile::Entry {
            next.push((x, y));
          }

          if *t == Tile::Floor {
            next.push((x, y));
          }

          if let Tile::Key(_) = *t {
            next.push((x, y));
          }

          if let Tile::Door(d) = *t {
            if collected_keys.contains(&d.to_lowercase().to_string().chars().next().unwrap()) {
              next.push((x, y));
            }
          }
        }
      }
      next.into_iter().map(|c| (c, 1))
    }, |&x| x == end)
  }

  fn can_reach(&self, start: Coords, end: Coords) -> bool {
    self.shortest_path(start, end).is_some()
  }

  fn get_coords(&self, tile: Tile) -> Option<Coords> {
    for (c, t) in self.map.iter() {
      if *t == tile {
        return Some(*c);
      }
    }

    None
  }

  fn reachable_keys(&self, start: Coords) -> HashSet<char> {
    let mut result = HashSet::new();

    for &key in &self.keys {
      let key_tile = Tile::Key(key);
      let coords = self.get_coords(key_tile);
      if coords.is_none() {
        continue;
      }

      let coords = coords.unwrap();
      if self.can_reach(start, coords) {
        result.insert(key);
      }
    }

    result
  }

  fn traveling_santa(&mut self, start: Coords) {
    let map = &self.map;
    let collected_keys = &mut self.collected_keys;
    let neighbor = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let all = dijkstra_all(&start, |&(px, py)| {
      let mut next = Vec::new();

      if let Some(Tile::Key(key)) = map.get(&(px, py)) {
        println!("Collecting key {}", key);
        collected_keys.insert(*key);
      }

      for n in neighbor.iter() {
        let x = px + n.0;
        let y = py + n.1;
        if let Some(t) = map.get(&(x, y)) {
          if *t == Tile::Entry {
            next.push((x, y));
          }

          if *t == Tile::Floor {
            next.push((x, y));
          }

          if let Tile::Key(_) = *t {
            next.push((x, y));
          }

          if let Tile::Door(d) = *t {
            println!("Checking door {}", d);
            println!("keys collected: {:?}", collected_keys);
            if collected_keys.contains(&d.to_lowercase().to_string().chars().next().unwrap()) {
              next.push((x, y));
            }
          }
        }
      }
      next.into_iter().map(|c| (c, 1))
    });
    println!("dijkstra all: {:?}", all);
  }
}

pub fn problem1() {
  // let input = include_str!("./data/input-1.txt");
  let input = include_str!("./data/example-1.txt");
  // let input = include_str!("./data/example-2.txt");
  let mut vault = Vault::new(input);

  println!("Keys: {:?} ({})", vault.keys, vault.keys.len());

  vault.traveling_santa(vault.entry);

  // let reachable = vault.reachable_keys(vault.entry);
  // println!("Reachable keys: {:?}", reachable);
  // println!("Vault: {:?}", vault.map);
}

pub fn problem2() {

}