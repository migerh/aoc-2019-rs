fn isa_interpreter(instructions: &mut Vec<i32>, input: i32) -> i32 {
  let mut ip = 0;
  let mut op = instructions[ip];

  let jump_table: Vec<usize> = vec![1, 4, 4, 2, 2, 3, 3, 4, 4];
  let param_table: Vec<u32> = vec![1, 2, 2, 0, 1, 2, 2, 2, 2];
  let mut outputs = vec![];

  while op != 99 {
    let main_op = op % 10;
    let num_params = param_table[main_op as usize];

    let mut operands = vec![];
    for i in 0..num_params {
      let mode = op / 10i32.pow(i + 2) % 10;
      let param = instructions[ip + i as usize + 1];
      let operand = if mode == 0 {
        instructions[param as usize]
      } else {
        param
      };
      operands.push(operand);
    }
    let result_index = instructions[ip + num_params as usize + 1] as usize;

    println!("ip: {}, op: {}, operands: {:?}, r: {}", ip, op, operands, result_index);

    let mut jumped = false;
    match main_op {
      1 => {
        instructions[result_index] = operands[0] + operands[1]
      },
      2 => {
        instructions[result_index] = operands[0] * operands[1]
      },
      3 => {
        instructions[result_index] = input
      },
      4 => {
        println!("Output! {}", operands[0]);
        outputs.push(operands[0]);
      },
      5 => {
        if operands[0] != 0 {
          ip = operands[1] as usize;
          jumped = true;
        }
      },
      6 => {
        if operands[0] == 0 {
          ip = operands[1] as usize;
          jumped = true;
        }
      },
      7 => {
        instructions[result_index] = if operands[0] < operands[1] {
          1
        } else {
          0
        };
      },
      8 => {
        instructions[result_index] = if operands[0] == operands[1] {
          1
        } else {
          0
        };
      },
      _ => panic!("At the math disco!")
    };

    if !jumped {
      ip += jump_table[main_op as usize];
    }
    op = instructions[ip];
  }

  println!("{:?}", outputs);

  outputs[outputs.len() - 1]
}

pub fn problem1() -> i32 {
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
  result
}

pub fn problem2() -> i32 {
  let input = include_str!("./data/input-1.txt");
  let mut opcodes = input
    .split(",")
    .filter(|v| *v != "\n")
    .filter(|v| *v != "")
    .map(|v| {
      v.parse::<i32>().unwrap()
    })
    .collect::<Vec<_>>();

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