fn load_input() -> Vec<i32> {
  include_str!("./data/input-1.txt")
    .split("")
    .filter(|v| *v != "")
    .filter(|v| *v != "\n")
    .map(|v| v.parse::<i32>().unwrap())
    .collect::<Vec<_>>()
}

fn count(slice: &[i32], value: i32) -> usize {
  slice.iter().filter(|v| **v == value).count()
}

fn image_stats() -> (usize, usize, usize) {
  (25, 6, 25 * 6)
}

pub fn problem1() -> usize {
  let raw_data = load_input();
  let (_, _, layer_size) = image_stats();
  let mut layer_stats = vec![];

  for layer in raw_data.chunks(layer_size) {
    let num_of_0 = count(layer, 0);
    let num_of_1 = count(layer, 1);
    let num_of_2 = count(layer, 2);
    layer_stats.push((num_of_0, num_of_1, num_of_2));
  }

  layer_stats.sort_by(|a, b| a.0.cmp(&b.0));
  let result = layer_stats[0].1 * layer_stats[0].2;
  println!("result: {}", result);
  result
}

pub fn problem2() {
  let raw_data = load_input();
  let (width, height, layer_size) = image_stats();

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
    assert_eq!(problem1(), 1935);
  }
}