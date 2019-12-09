use super::intcode::isa_interpreter;

fn parse_input(input: &str) -> Vec<i64> {
  input.split(",")
    .filter(|v| *v != "\n")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i64>().unwrap())
    .collect::<Vec<_>>()
}

fn run_with_input(input: i64) -> i64 {
  let code = include_str!("./data/input-1.txt");
  let mut instructions = parse_input(code);
  isa_interpreter(&mut instructions, input)
}

pub fn problem1() {
  let result = run_with_input(1);
  println!("Result 9-1: {}", result);
}

pub fn problem2() {
  let result = run_with_input(2);
  println!("Result 9-2: {}", result);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example1() {
    let mut instructions = parse_input("1102,34915192,34915192,7,4,7,99,0");
    assert_eq!(isa_interpreter(&mut instructions, 1), 1219070632396864);
  }

  #[test]
  fn problem1_example2() {
    // let mut instructions = parse_input("108,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    // assert_eq!(isa_interpreter(&mut instructions, 1), 99);
  }

  #[test]
  fn problem1_example3() {
    let mut instructions = parse_input("104,1125899906842624,99");
    assert_eq!(isa_interpreter(&mut instructions, 1), 1125899906842624);
  }
}