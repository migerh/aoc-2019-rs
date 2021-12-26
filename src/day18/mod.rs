use crate::utils::ParseError;
use pathfinding::prelude::*;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Write};
use std::str::FromStr;

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

impl Display for Vault {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut mx = (i64::MIN, i64::MIN);

        for (p, _) in &self.map {
            mx.0 = max(mx.0, p.0);
            mx.1 = max(mx.1, p.1);
        }

        w.write_char('\n')?;
        for y in 0..=mx.1 {
            for x in 0..=mx.0 {
                if let Some(t) = self.map.get(&(x, y)) {
                    let c = match t {
                        Tile::Door(d) => *d,
                        Tile::Floor => '.',
                        Tile::Key(k) => *k,
                        Tile::Entry => '@',
                        Tile::Wall => '#',
                    };
                    w.write_char(c)?;
                }
            }
            w.write_char('\n')?;
        }

        Ok(())
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

    fn unreachable_keys(&self, start: Coords) -> Vec<char> {
        let neighbor = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut queue = vec![start];
        let mut visited = HashSet::new();
        let mut reachable_keys = HashSet::new();

        while let Some(q) = queue.pop() {
            if visited.contains(&q) {
                continue;
            }

            visited.insert(q);
            for n in &neighbor {
                let nx = q.0 + n.0;
                let ny = q.1 + n.1;

                if let Some(t) = self.map.get(&(nx, ny)) {
                    if *t != Tile::Wall {
                        queue.push((nx, ny));
                    }

                    if let Tile::Key(k) = *t {
                        reachable_keys.insert(k);
                    }
                }
            }
        }

        self.keys.difference(&reachable_keys).map(|v| *v).collect::<Vec<_>>()
    }

    fn traveling_santa(&mut self, start: Coords, keys: Vec<char>) -> Option<usize> {
        let map = &self.map;
        let target = self.keys.len();
        let neighbor = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let start_state = (start, keys);

        let all = dijkstra(
            &start_state,
            |((px, py), keys)| {
                let mut next = Vec::new();
                for n in neighbor.iter() {
                    let x = px + n.0;
                    let y = py + n.1;
                    if let Some(t) = map.get(&(x, y)) {
                        if *t == Tile::Entry {
                            next.push(((x, y), keys.clone()));
                        }

                        if *t == Tile::Floor {
                            next.push(((x, y), keys.clone()));
                        }

                        if let Tile::Key(k) = *t {
                            let mut new_keys = keys.clone();
                            if !new_keys.contains(&k) {
                                new_keys.push(k);
                            }
                            // I don't know why but sorting the list of keys
                            // prevents infinite loops on some inputs like example
                            // 4 and my real input...
                            new_keys.sort();
                            next.push(((x, y), new_keys));
                        }

                        if let Tile::Door(d) = *t {
                            if keys.contains(&d.to_lowercase().next().unwrap()) {
                                next.push(((x, y), keys.clone()));
                            }
                        }
                    }
                }
                next.into_iter().map(|c| (c, 1))
            },
            |(_, keys)| keys.len() >= target,
        );

        all.map(|(_, distance)| distance)
    }
}

#[aoc(day18, part1)]
pub fn problem1(vault: &Vault) -> Result<usize, ParseError> {
    let mut vault = vault.clone();
    vault
        .traveling_santa(vault.entry, vec![])
        .ok_or(ParseError::new("Could not determine a path"))
}

#[aoc(day18, part2)]
pub fn problem2(vault: &Vault) -> Result<usize, ParseError> {
    let mut vault = vault.clone();

    // patch vault
    let (xe, ye) = vault.entry;

    // add walls
    vault.map.entry(vault.entry).and_modify(|v| *v = Tile::Wall);
    vault
        .map
        .entry((xe, ye + 1))
        .and_modify(|v| *v = Tile::Wall);
    vault
        .map
        .entry((xe, ye - 1))
        .and_modify(|v| *v = Tile::Wall);
    vault
        .map
        .entry((xe + 1, ye))
        .and_modify(|v| *v = Tile::Wall);
    vault
        .map
        .entry((xe - 1, ye))
        .and_modify(|v| *v = Tile::Wall);

    // add entries
    vault
        .map
        .entry((xe - 1, ye - 1))
        .and_modify(|v| *v = Tile::Entry);
    vault
        .map
        .entry((xe + 1, ye - 1))
        .and_modify(|v| *v = Tile::Entry);
    vault
        .map
        .entry((xe - 1, ye + 1))
        .and_modify(|v| *v = Tile::Entry);
    vault
        .map
        .entry((xe + 1, ye + 1))
        .and_modify(|v| *v = Tile::Entry);

    let start_coords = [
        (xe - 1, ye - 1),
        (xe + 1, ye - 1),
        (xe - 1, ye + 1),
        (xe + 1, ye + 1),
    ];

    let result = start_coords.into_iter().map(|c| {
            let keys_from_other_sections = vault.unreachable_keys(c);
            vault
                .traveling_santa(c, keys_from_other_sections)
                .ok_or(ParseError::new("Could not determine a path"))
        })
        .collect::<Result<Vec<_>, ParseError>>()?
        .into_iter()
        .sum();
    Ok(result)
}
