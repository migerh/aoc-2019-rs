use crate::utils::ParseError;
use std::str::FromStr;
use pathfinding::prelude::*;
use std::collections::{HashMap, HashSet};

type Coords = (i64, i64);

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Floor,
    Wall,
    Entry,
    Key(char),
    Door(char),
}

#[derive(Debug, Clone)]
pub struct Vault {
    map: HashMap<Coords, Tile>,
    keys: HashSet<char>,
    collected_keys: HashSet<char>,
    entry: Coords,
}

impl FromStr for Vault {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut map = HashMap::new();
        let mut keys = HashSet::new();
        let collected_keys = HashSet::new();
        let mut entry = (0, 0);

        let lines = s.lines();
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                *map.entry((x as i64, y as i64)).or_insert(Tile::Wall) = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    '@' => {
                        entry = (x as i64, y as i64);
                        Tile::Entry
                    }
                    v if Vault::is_door(v) => Tile::Door(c),
                    v if Vault::is_key(v) => {
                        keys.insert(v);
                        Tile::Key(c)
                    }
                    _ => panic!("Unknown symbol: {}", c),
                };
            }
        }

        Ok(Vault {
            map,
            keys,
            entry,
            collected_keys,
        })
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Result<Vault, ParseError> {
    Vault::from_str(input)
}

impl Vault {
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

        dijkstra(
            &start,
            |&(px, py)| {
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
                            if collected_keys
                                .contains(&d.to_lowercase().to_string().chars().next().unwrap())
                            {
                                next.push((x, y));
                            }
                        }
                    }
                }
                next.into_iter().map(|c| (c, 1))
            },
            |&x| x == end,
        )
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
                        if collected_keys
                            .contains(&d.to_lowercase().to_string().chars().next().unwrap())
                        {
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

#[aoc(day18, part1)]
pub fn problem1(vault: &Vault) -> Result<usize, ParseError> {
    let mut vault = vault.clone();
    println!("Keys: {:?} ({})", vault.keys, vault.keys.len());

    vault.traveling_santa(vault.entry);

    // let reachable = vault.reachable_keys(vault.entry);
    // println!("Reachable keys: {:?}", reachable);
    // println!("Vault: {:?}", vault.map);
    Ok(0)
}

#[aoc(day18, part2)]
pub fn problem2(vault: &Vault) -> usize {
    0
}
