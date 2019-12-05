fn isa_interpreter(instructions: &mut Vec<i32>, input: i32) -> i32 {
  let mut ip = 0;
  let mut op = instructions[ip];

  let jump_table: Vec<usize> = vec![1, 4, 4, 2, 2];
  let mut outputs = vec![];

  println!("{:?}", instructions);

  while op != 99 {
    let param_mode1 = op / 100 % 10;
    let param_mode2 = op / 1000 % 10;

    let param1 = instructions[ip + 1] as usize;
    let param2 = instructions[ip + 2] as usize;
    let result_address = instructions[ip + 3] as usize;

    // for (idx, i) in instructions.iter().enumerate() {
    //   println!("{}: {}", idx, i);
    // }
    println!("ip: {}, op: {}, p1: {} ({}), p2: {} ({}), r: {}", ip, op, param1, param_mode1, param2, param_mode2, result_address);

    let main_op = op % 10;

    let operand1 = if param_mode1 == 0 {
      instructions[param1]
    } else {
      param1 as i32
    };

    let operand2 = if param_mode2 == 0 && main_op <= 2 {
      instructions[param2]
    } else {
      param2 as i32
    };

    println!("ip: {}, op: {}, o1: {}, o2: {}, r: {}", ip, op, operand1, operand2, result_address);

    if main_op <= 2 {
      instructions[result_address] = match main_op {
        1 => operand1 + operand2,
        2 => operand1 * operand2,
        _ => panic!("At the math disco!")
      }
    } else {
      instructions[param1] = match main_op {
        3 => input,
        4 => {
          println!("Output! {}", operand1);
          outputs.push(operand1);
          instructions[param1]
        },
      v => panic!("at the disco: {}", v)
      };
    }

    ip += jump_table[main_op as usize];
    op = instructions[ip];
  }

  println!("{:?}", outputs);

  outputs[outputs.len() - 1]
}

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let mut opcodes = input
    .split(",")
    .filter(|v| *v != "\n")
    .filter(|v| *v != "")
    .map(|v| {
      v.parse::<i32>().unwrap()
    })
    .collect::<Vec<_>>();

  let result = isa_interpreter(&mut opcodes, 1);
  println!("result: {}", result);
}

pub fn problem2() {
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