use std::sync::mpsc::{Receiver, channel};
use super::intcode::{parse_instructions, isa_interpreter_async, isa_interpreter_mi};

fn monitor(recv: Receiver<i64>) -> Vec<char> {
  let mut output = vec![];
  loop {
    let value = recv.recv();
    if let Err(err) = value {
      println!("Err in monitor thread while reading data: {}", err);
      break;
    } else if let Ok(value) = value {
      output.push(char::from(value as u8));
    }
  }

  output
}

fn hash(map: &str) -> usize {
  let lines: Vec<_> = map
    .lines()
    .map(|v| v.chars().collect::<Vec<_>>())
    .filter(|v| v.len() > 0)
    .collect();

  let mut intersections = vec![];

  for y in 1..lines.len()-1 {
    for x in 1..lines[0].len()-1 {
      let mut relevant_tiles = vec![];
      relevant_tiles.push(lines[y][x]);
      relevant_tiles.push(lines[y][x-1]);
      relevant_tiles.push(lines[y][x+1]);
      relevant_tiles.push(lines[y+1][x]);
      relevant_tiles.push(lines[y-1][x]);
      let check: String = relevant_tiles.iter().collect();
      if check == "#####" {
        intersections.push((x, y));
      }
    }
  }

  let mut result = 0;
  for p in intersections {
    result += p.0 * p.1;
  }

  result
}

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let instructions = parse_instructions(&input);

  let (_, isa_recv) = channel();
  let (mon_send, mon_recv) = channel();
  let thread_monitor = std::thread::spawn(move || {
    monitor(mon_recv)
  });
  let isa_thread = std::thread::spawn(move || {
    isa_interpreter_async(instructions, isa_recv, mon_send)
  });

  let map = thread_monitor.join().unwrap();
  isa_thread.join().unwrap();

  let map: String = map.iter().collect();
  // println!("{}", map);

  let result = hash(&map);
  println!("Result 17-1: {}", result);
}

fn str_to_ascii(input: &str) -> Vec<i64> {
  input
    .chars()
    .map(|v| v as i64)
    .collect::<Vec<_>>()
}

pub fn problem2() {
  // Solved manually by retracing the labyrinth
  // The recurring patterns emerge pretty quickly
  //
  // A L,4,L,10,L,6,
  // A L,4,L,10,L,6,
  // B L,6,L,4,R,8,R,8,
  // C L,6,R,8,L,10,L,8,L,8,
  // A L,4,L,10,L,6,
  // C L,6,R,8,L,10,L,8,L,8,
  // B L,6,L,4,R,8,R,8,
  // C L,6,R,8,L,10,L,8,L,8,
  // A L,4,L,10,L,6,
  // B L,6,L,4,R,8,R,8,
  //
  // Main: A,A,B,C,A,C,B,C,A,B

  let main = "A,A,B,C,A,C,B,C,A,B\n";
  let a = "L,4,L,10,L,6\n";
  let b = "L,6,L,4,R,8,R,8\n";
  let c = "L,6,R,8,L,10,L,8,L,8\n";

  let debug = "n\n";

  let mut input = vec![];
  input.append(&mut str_to_ascii(main));
  input.append(&mut str_to_ascii(a));
  input.append(&mut str_to_ascii(b));
  input.append(&mut str_to_ascii(c));
  input.append(&mut str_to_ascii(debug));

  let code = include_str!("./data/input-1.txt");
  let mut instructions = parse_instructions(&code);
  // patch the code
  instructions[0] = 2;

  let result = isa_interpreter_mi(&mut instructions, &input);
  println!("Result 17-2: {}", result);
}
