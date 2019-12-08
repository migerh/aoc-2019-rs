use std::collections::HashMap;

fn load_input() -> Vec<i32> {
  include_str!("./data/input-1.txt")
    .split("")
    .filter(|v| *v != "")
    .filter(|v| *v != "\n")
    .map(|v| v.parse::<i32>().unwrap())
    .collect::<Vec<_>>()
}

pub fn problem1() {
  let raw_data = load_input();
  let height = 6;
  let width = 25;
  let layer_size = height * width;
  let mut layer_stats = vec![];

  for layer in raw_data.chunks(layer_size) {
    let num_of_0 = layer.iter().filter(|v| **v == 0).count();
    let num_of_1 = layer.iter().filter(|v| **v == 1).count();
    let num_of_2 = layer.iter().filter(|v| **v == 2).count();
    layer_stats.push((num_of_0, num_of_1, num_of_2));
  }

  let mut smallest_idx = 0;
  let mut smallest_0 = 200;
  for (idx, layer) in layer_stats.iter().enumerate() {
    if smallest_0 > layer.0 {
      smallest_0 = layer.0;
      smallest_idx = idx;
    }
  }

  println!("smallest num of 0 in layer {}", smallest_idx);
  let result = layer_stats[smallest_idx].1 * layer_stats[smallest_idx].2;
  println!("result: {}", result);
}

pub fn problem2() {
  let raw_data = load_input();
  let width = 25;
  let height = 6;
  let layer_size = height * width;

  let mut map = vec![vec![2; width]; height];
  for layer in raw_data.chunks(layer_size) {
    for (idx, pixel) in layer.iter().enumerate() {
      let coords = (idx % width, idx / width);
      if map[coords.1][coords.0] == 2 {
        map[coords.1][coords.0] = *pixel;
      }
    }
  }

  for row in map {
    for pixel in row {
      if pixel == 1 {
        print!("X");
      } else {
        print!(" ");
      }
    }
    println!("");
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example1() {
    assert_eq!(1, 1);
  }
}