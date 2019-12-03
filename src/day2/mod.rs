fn isa_interpreter(instructions: &mut Vec<i32>) -> i32 {
  let mut ip = 0;
  let mut op = instructions[ip];
  while op != 99 {
    let operand1 = instructions[ip + 1] as usize;
    let operand2 = instructions[ip + 2] as usize;
    let result = instructions[ip + 3] as usize;

    instructions[result] = if op == 1 {
      instructions[operand1] + instructions[operand2]
    } else {
      instructions[operand1] * instructions[operand2]
    };

    ip += 4;
    op = instructions[ip];
  }

  instructions[0]
}

fn patch_and_interpret(instructions: &str, noun: i32, verb: i32) -> i32 {
  let mut instructions = instructions
    .split(",")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i32>().unwrap())
    .collect::<Vec<_>>();

  instructions[1] = noun;
  instructions[2] = verb;

  isa_interpreter(&mut instructions)
}

fn patch_and_interpret_problem1(instructions: &str) -> i32 {
  let mut instructions = instructions
    .split(",")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i32>().unwrap())
    .collect::<Vec<_>>();

  instructions[1] = 12;
  instructions[2] = 2;

  isa_interpreter(&mut instructions)
}

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let result = patch_and_interpret_problem1(input);
  println!("result: {}", result);
}

pub fn problem2() {
  let expected_outcome: i32 = 19690720;

  for n in 0..100 {
    for v in 0..100 {
      let input = include_str!("./data/input-1.txt");
      let result = patch_and_interpret(input, n, v);
      if result == expected_outcome {
        println!("Found noun {} and verb {} to produce {}. ⇒ Result is {}", n, v, expected_outcome, 100*n + v);
      }
    }
  }
  println!("tbd");
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example1() {
    // assert_eq!(patch_and_interpret_problem1("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);

    // assert_eq!(patch_and_interpret_problem1("1,0,0,0,99"), 2);
  }

  #[test]
  fn problem2_example1() {
    assert_eq!(2, 2);
  }
}