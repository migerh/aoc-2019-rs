use std::cmp::{max, min};
use std::collections::HashMap;
use std::iter::Iterator;
use std::sync::mpsc::{channel, Receiver, Sender};

use super::intcode::{isa_interpreter_async, parse_instructions};
use pathfinding::prelude::dijkstra;
use rand::prelude::*;

type Coords = (i64, i64);

#[derive(Debug, PartialEq, Eq)]
enum Tile {
  Wall,
  Floor,
  Oxygen,
  Todo,
}

#[derive(Debug, Clone)]
enum Command {
  East,
  North,
  West,
  South,
}

struct Neighbors {
  position: Coords,
  current: i32,
}

impl Neighbors {
  fn iter(position: Coords) -> Neighbors {
    let current = 0;
    Neighbors { position, current }
  }
}

impl Iterator for Neighbors {
  type Item = (Coords, Command);

  fn next(&mut self) -> Option<Self::Item> {
    let (result, new_current) = match self.current {
      0 => (
        Some(((self.position.0 + 1, self.position.1 + 0), Command::East)),
        1,
      ),
      1 => (
        Some(((self.position.0 + 0, self.position.1 + 1), Command::North)),
        2,
      ),
      2 => (
        Some(((self.position.0 - 1, self.position.1 + 0), Command::West)),
        3,
      ),
      3 => (
        Some(((self.position.0 + 0, self.position.1 - 1), Command::South)),
        4,
      ),
      _ => (None, 4),
    };
    self.current = new_current;
    result
  }
}

struct Robot {
  position: Coords,
  backlog: Vec<Coords>,
  map: HashMap<Coords, Tile>,
}

impl Robot {
  fn new() -> Robot {
    let position = (0, 0);
    let mut map = HashMap::new();
    let backlog = vec![];

    map.entry(position).or_insert(Tile::Floor);

    Robot {
      position,
      backlog,
      map,
    }
  }

  fn fill_todo(&mut self) {
    for neighbor in Neighbors::iter(self.position) {
      self.map.entry(neighbor.0).or_insert(Tile::Todo);
    }
  }

  fn find_neighboring_tile_of_type(&mut self, tile: Tile) -> Option<(Coords, Command)> {
    for neighbor in Neighbors::iter(self.position) {
      let entry = self.map.entry(neighbor.0).or_insert(Tile::Todo);
      if *entry == tile {
        return Some(neighbor);
      }
    }

    None
  }

  fn walk(&self, command: Command) -> Coords {
    let update = match command {
      Command::North => (0, 1),
      Command::South => (0, -1),
      Command::West => (-1, 0),
      Command::East => (1, 0),
    };

    (self.position.0 + update.0, self.position.1 + update.1)
  }

  fn shortest_path(&self, start: Coords, end: Coords) -> Option<Vec<Coords>> {
    let result = dijkstra(&start, |pos| get_neighbors(pos, &self.map), |&p| p == end);
    if let Some(result) = result {
      Some(result.0)
    } else {
      None
    }
  }

  fn get_command(&self, to: Coords) -> Option<Command> {
    let dx = self.position.0 - to.0;
    let dy = self.position.1 - to.1;

    match (dx, dy) {
      (1, 0) => Some(Command::West),
      (0, 1) => Some(Command::North),
      (-1, 0) => Some(Command::East),
      (0, -1) => Some(Command::South),
      _ => None
    }
  }

  fn find_suitable_command(&mut self) -> Option<(Coords, Command)> {
    self.fill_todo();

    if !self.backlog.is_empty() {
      if let Some(next) = self.backlog.pop() {
        if let Some(command) = self.get_command(next) {
          return Some((next, command));
        }
      }
    }

    // if let Some(todo_neighbor) = self.find_neighboring_tile_of_type(Tile::Todo) {
    //   return Some(todo_neighbor);
    // }

    // find next todo:
    // if let Some(goal) = self.todo.pop() {
    let mut next_todo: Option<Coords> = None;
    for (k, v) in self.map.iter() {
      if *v == Tile::Todo {
        next_todo = Some(*k);
        break;
      }
    }

    if let Some(goal) = next_todo {
      if let Some(path) = self.shortest_path(goal, self.position) {
        self.backlog = path;
        self.backlog.pop();
        return self.find_suitable_command();
      }
    } else {
      println!("Could not find a todo in our map!");
    }

    None

    // random walk:
    // println!("RANDOM WALK!");
    // let mut rng = rand::thread_rng();
    // let dir: u8 = rng.gen::<u8>() / 64u8;

    // let command = match dir {
    //   0 => Command::North,
    //   1 => Command::South,
    //   2 => Command::West,
    //   3 => Command::East,
    //   _ => Command::East,
    // };
    // let new_pos = self.walk(command.clone());
    // Some((new_pos, command))

    // for (pos, _) in Neighbors::iter(self.position) {
    //   if *self.map.entry(pos).or_insert(Tile::Todo) == Tile::Todo {
    //     self.todo.push(pos);
    //   }
    // }

    // if self.todo.is_empty() {
    //   return None;
    // }

    // // find shortest path to next todo:
    // if let Some(next) = self.todo.pop() {

    // }

    // else if let Some(floor_neighbor) = self.find_neighboring_tile_of_type(Tile::Floor) {
    //   Some(floor_neighbor)
    // } else {
    //   None
    // }
    // None
  }
}

fn command_to_isa(command: Command) -> i64 {
  match command {
    Command::North => 1,
    Command::South => 2,
    Command::West => 3,
    Command::East => 4,
  }
}

fn isa_to_tile(isa_tile: i64) -> Tile {
  match isa_tile {
    0 => Tile::Wall,
    1 => Tile::Floor,
    2 => Tile::Oxygen,
    _ => Tile::Todo,
  }
}

fn robot_brain(send: Sender<i64>, recv: Receiver<i64>) -> HashMap<Coords, Tile> {
  let mut robot = Robot::new();
  let mut next_coords;

  loop {
    if let Some(command) = robot.find_suitable_command() {
      next_coords = command.0;
      send.send(command_to_isa(command.1)).unwrap();
    } else {
      println!("Could not find suitable command for robot");
      break;
    }

    let status = recv.recv();
    if let Ok(status) = status {
      if status == 99 {
        break;
      }
      if status == 0 {
        *robot.map.entry(next_coords).or_insert(Tile::Wall) = Tile::Wall;
      } else if status == 1 {
        *robot.map.entry(next_coords).or_insert(Tile::Floor) = Tile::Floor;
        robot.position = next_coords;
      } else if status == 2 {
        *robot.map.entry(next_coords).or_insert(Tile::Oxygen) = Tile::Oxygen;
        robot.position = next_coords;
      }
    } else if let Err(err) = status {
      println!("Error while receiving status update: {}", err);
    }
  }

  robot.map
}

fn print_map(map: &HashMap<Coords, Tile>, pos: &Coords) {
  let mut mac = (0, 0);
  let mut mic = (0, 0);
  for (k, _) in map.iter() {
    mac = (max(mac.0, k.0), max(mac.1, k.1));
    mic = (min(mic.0, k.0), min(mic.1, k.1));
  }
  let dim = (mac.0 - mic.0, mac.1 - mic.1);

  let mut canvas = vec![vec![' '; dim.0 as usize + 1]; dim.1 as usize + 1];
  for (k, v) in map.iter() {
    canvas[(k.1 - mic.1) as usize][(k.0 - mic.0) as usize] = match *v {
      Tile::Wall => '#',
      Tile::Floor => '.',
      Tile::Oxygen => 'D',
      Tile::Todo => 't',
    };
  }

  if dim.0 >= pos.0 - mic.0 && dim.1 >= pos.1 - mic.1 {
    canvas[(pos.1 - mic.1) as usize][(pos.0 - mic.0) as usize] = 'R';
  }

  if dim.0 >= 0 - mic.0 && dim.1 >= 0 - mic.1 {
    canvas[(0 - mic.1) as usize][(0 - mic.0) as usize] = 'X';
  }

  for line in canvas {
    for c in line {
      print!("{}", c);
    }
    println!("");
  }
}

fn get_neighbors(pos: &Coords, map: &HashMap<Coords, Tile>) -> Vec<(Coords, i64)> {
  let mut result = vec![];
  for neighbor in Neighbors::iter(*pos) {
    if let Some(tile) = map.get(&neighbor.0) {
      if *tile != Tile::Wall {
        result.push((neighbor.0, 1));
      }
    }
  }
  result
}

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let instructions = parse_instructions(input);

  let (isa_send, isa_recv) = channel();
  let (robo_send, robo_recv) = channel();

  let isa_thread =
    std::thread::spawn(move || isa_interpreter_async(instructions, isa_recv, robo_send));
  let robo_thread = std::thread::spawn(move || robot_brain(isa_send, robo_recv));

  let map = robo_thread.join().unwrap();
  let isa_result = isa_thread.join();
  if let Err(_) = isa_result {
    println!("ISA thread closed with err.");
  }

  let mut oxygen = (0, 0);
  for (k, v) in map.iter() {
    if *v == Tile::Oxygen {
      oxygen = *k;
      break;
    }
  }
  let result = dijkstra(&(0, 0), |pos| get_neighbors(pos, &map), |&p| p.0 == oxygen.0 && p.1 == oxygen.1);

  println!("Result 15-1: {}", result.unwrap().1);

  print_map(&map, &(0, 0));
}

pub fn problem2() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example1() {
    assert_eq!(1, 1);
  }
}
