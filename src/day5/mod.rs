use super::intcode::{isa_interpreter};

fn load_code() -> Vec<i32> {
  let input = include_str!("./data/input-1.txt");
  let opcodes = input
    .split(",")
    .filter(|v| *v != "\n")
    .filter(|v| *v != "")
    .map(|v| {
      v.parse::<i32>().unwrap()
    })
    .collect::<Vec<_>>();

  opcodes
}

pub fn problem1() -> i32 {
  let mut opcodes = load_code();

  let result = isa_interpreter(&mut opcodes, 1);
  println!("result: {}", result);
  result
}

pub fn problem2() -> i32 {
  let mut opcodes = load_code();

  let result = isa_interpreter(&mut opcodes, 5);
  println!("result: {}", result);
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_solution() {
    assert_eq!(problem1(), 9025675);
  }

  #[test]
  fn problem2_example1() {
    let mut instructions = vec![3,9,8,9,10,9,4,9,99,-1,8];
    let result = isa_interpreter(&mut instructions, 8);
    assert_eq!(result, 1);
  }

  #[test]
  fn problem2_example2() {
    let mut instructions = vec![3,9,7,9,10,9,4,9,99,-1,8];
    let result = isa_interpreter(&mut instructions, 4);
    assert_eq!(result, 1);
  }

  #[test]
  fn problem2_example3() {
    let mut instructions = vec![3,3,1108,-1,8,3,4,3,99];
    let result = isa_interpreter(&mut instructions, 8);
    assert_eq!(result, 1);
  }

  #[test]
  fn problem2_example4() {
    let mut instructions = vec![3,3,1107,-1,8,3,4,3,99];
    let result = isa_interpreter(&mut instructions, 4);
    assert_eq!(result, 1);
  }

  #[test]
  fn problem2_example5() {
    let mut instructions = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let result = isa_interpreter(&mut instructions, 3);
    assert_eq!(result, 1);
  }

  #[test]
  fn problem2_example6() {
    let mut instructions = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    let result = isa_interpreter(&mut instructions, 8);
    assert_eq!(result, 1000);
  }
}