use std::sync::mpsc::{channel, Sender, Receiver};
use std::cmp::{min, max};
use std::collections::HashMap;
use super::intcode::{isa_interpreter_async, parse_instructions};

type Channel<T> = (Sender<T>, Receiver<T>);
type Coords = (i64, i64);

fn robo_brain((send, recv): Channel<i64>, map: HashMap<Coords, i64>) -> HashMap<Coords, i64> {
  let mut position = (0, 0);
  let mut direction = (0, -1);
  let mut map = map;

  loop {
    let color = map.entry(position).or_insert(0i64);
    send.send(*color).unwrap();

    let new_color = recv.recv().unwrap();
    if new_color == 99 {
      break;
    }
    let rotation = recv.recv().unwrap();

    if rotation == 99 {
      break;
    }

    *color = new_color;
    direction = match (direction, rotation) {
      ((0, -1), 0) => (-1, 0),
      ((0, 1), 0) => (1, 0),
      ((0, -1), 1) => (1, 0),
      ((0, 1), 1) => (-1, 0),
      ((1, 0), 0) => (0, -1),
      ((-1, 0), 0) => (0, 1),
      ((1, 0), 1) => (0, 1),
      ((-1, 0), 1) => (0, -1),
      _ => panic!("invalid direction/rotation combo: {:?}, {}", direction, rotation)
    };
    position = (position.0 + direction.0, position.1 + direction.1);
  }

  map
}

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let instructions = parse_instructions(input);

  let (robo_send, robo_recv) = channel();
  let (isa_send, isa_recv) = channel();

  let map = HashMap::new();
  let robo_thread = std::thread::spawn(move || {
    robo_brain((isa_send, robo_recv), map)
  });

  let robo_send_for_isa = robo_send.clone();
  let isa_thread = std::thread::spawn(move || {
    isa_interpreter_async(instructions, isa_recv, robo_send_for_isa)
  });

  isa_thread.join().unwrap();
  // terminate the robo brain thread
  robo_send.send(99).unwrap();
  let result = robo_thread.join().unwrap().len();
  println!("result 11-1: {}", result);
}

pub fn problem2() {
  let input = include_str!("./data/input-1.txt");
  let instructions = parse_instructions(input);

  println!("running 11-2");

  let (robo_send, robo_recv) = channel();
  let (isa_send, isa_recv) = channel();

  let mut map = HashMap::new();
  map.entry((0, 0)).or_insert(1i64);
  let robo_thread = std::thread::spawn(move || {
    robo_brain((isa_send, robo_recv), map)
  });

  let robo_send_for_isa = robo_send.clone();
  let isa_thread = std::thread::spawn(move || {
    isa_interpreter_async(instructions, isa_recv, robo_send_for_isa)
  });

  isa_thread.join().unwrap();
  // terminate the robo brain thread
  robo_send.send(99).unwrap();
  let result = robo_thread.join().unwrap();

  let mut mac = (0, 0);
  let mut mic = (0, 0);
  for (k, _) in result.iter() {
    mac = (max(mac.0, k.0), max(mac.1, k.1));
    mic = (min(mic.0, k.0), min(mic.1, k.1));
  }

  println!("{:?}, {:?}", mic, mac);
  let mut canvas = vec![vec![' '; 50]; 6];
  for (k, v) in result.iter() {
    canvas[k.1 as usize][k.0 as usize] = if *v == 1 {
      'X'
    } else {
      ' '
    };
  }

  for line in canvas {
    for c in line {
      print!("{}", c);
    }
    println!("");
  }

  println!("result 11-2: {:?}", result);
}

#[cfg(test)]
mod test {
  #[test]
  fn problem1_example1() {

    assert_eq!(1, 1);
  }
}