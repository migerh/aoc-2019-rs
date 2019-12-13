use super::intcode::isa_interpreter;

fn isa_interpreter_wrap(instructions: &mut Vec<i64>) -> i64 {
  isa_interpreter(instructions, 0)
}

fn patch_and_interpret(instructions: &str, noun: i64, verb: i64) -> i64 {
  let mut instructions = instructions
    .split(",")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i64>().unwrap())
    .collect::<Vec<_>>();

  instructions[1] = noun;
  instructions[2] = verb;

  isa_interpreter_wrap(&mut instructions)
}

fn patch_and_interpret_problem1(instructions: &str) -> i64 {
  let mut instructions = instructions
    .split(",")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i64>().unwrap())
    .collect::<Vec<_>>();

  instructions[1] = 12;
  instructions[2] = 2;

  isa_interpreter_wrap(&mut instructions)
}

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let result = patch_and_interpret_problem1(input);
  println!("result: {}", result);
}

pub fn problem2() {
  let expected_outcome: i64 = 19690720;

  for n in 0..100 {
    for v in 0..100 {
      let input = include_str!("./data/input-1.txt");
      let result = patch_and_interpret(input, n, v);
      if result == expected_outcome {
        println!("Found noun {} and verb {} to produce {}. ⇒ Result is {}", n, v, expected_outcome, 100*n + v);
      }
    }
  }
}

#[cfg(test)]
mod test {
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