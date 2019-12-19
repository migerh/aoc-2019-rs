use std::collections::HashMap;
use std::cmp::{min, max};
use super::intcode::{parse_instructions, isa_interpreter_mi};

type Coords = (i64, i64);

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let instructions = parse_instructions(&input);

  let mut counter = 0;
  for x in 0..50i64 {
    for y in 0..50i64 {
      counter += isa_interpreter_mi(&mut instructions.clone(), &vec![x, y]);
    }
  }

  println!("Result 19-1: {}", counter);
}

fn print_map(map: &HashMap<Coords, i64>) {
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
      0 => '.',
      1 => '#',
      _ => '?',
    };
  }

  for line in canvas {
    for c in line {
      print!("{}", c);
    }
    println!("");
  }
}

pub fn problem2() {
  let input = include_str!("./data/input-1.txt");
  let instructions = parse_instructions(&input);
  let mut beam_width = vec![];

  let mut start_pos = (0, 0);
  let mut map = HashMap::new();
  // determined manually, with trial and
  // error and some basic geometry
  for y in 950..10000i64 {
    let mut count_x_100 = 0;
    let mut first_x = -1;
    for x in 600..1000i64 {
      let is_tractored = isa_interpreter_mi(&mut instructions.clone(), &vec![x, y]);
      count_x_100 += is_tractored;
      if is_tractored == 1 && first_x == -1 {
        first_x = x;
      }
      *map.entry((x, y)).or_insert(0) = is_tractored;
    }

    beam_width.push((first_x, count_x_100));

    if beam_width.len() >= 100 {
      let start_last = beam_width[beam_width.len() - 1].0;
      let start_before = beam_width[beam_width.len() - 100].0;
      let len_before_100 = beam_width[beam_width.len() - 100].1;
      println!("diff_start is {}, len_before_100 is {}", start_last - start_before, len_before_100);

      if (start_last - start_before) + 100 <= len_before_100 {
        println!("We're done!");
        start_pos = (start_last, y-99);
        break;
      }
    }
  }

  // print_map(&map);
  // too low: 7720974
  // too high: 7721074
  let result = start_pos.0 * 10000 + start_pos.1;
  println!("Result 19-1: {}", result);
}