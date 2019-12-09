use std::sync::mpsc::{channel, Receiver, Sender};

pub fn isa_interpreter(instructions: &mut Vec<i64>, input: i64) -> i64 {
  isa_interpreter_mi(instructions, &vec![input])
}

pub fn isa_interpreter_mi(instructions: &mut Vec<i64>, input: &Vec<i64>) -> i64 {
  let (send, recv) = channel();
  for i in input {
    send.send(*i).unwrap();
  }
  isa_interpreter_mpsc(instructions, recv)
}

pub fn isa_interpreter_mpsc(instructions: &mut Vec<i64>, input: Receiver<i64>) -> i64 {
  let (send, _recv) = channel();
  isa_interpreter_async(instructions.clone(), input, send)
}

pub fn isa_interpreter_async(instructions: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) -> i64 {
  let mut ip = 0;
  let mut instructions = instructions.clone();
  // reserve 4MB for the intcode program
  instructions.resize(524288, 0);
  let mut op = instructions[ip];

  let jump_table: Vec<usize> = vec![1, 4, 4, 2, 2, 3, 3, 4, 4, 2];
  let param_table: Vec<u32> = vec![1, 2, 2, 0, 1, 2, 2, 2, 2, 1];
  let mut outputs = vec![];
  let mut relative_base = 0;

  while op != 99 {
    let main_op = op % 10;
    let num_params = param_table[main_op as usize];

    let mut operands = vec![];
    for i in 0..num_params {
      let mode = op / 10i64.pow(i + 2) % 10;
      let param = instructions[ip + i as usize + 1];
      let operand = if mode == 0 {
        instructions[param as usize]
      } else if mode == 1 {
        param
      } else if mode == 2 {
        instructions[(relative_base + param) as usize]
      } else {
        panic!("Unknown param mode: {}", mode);
      };
      operands.push(operand);
    }
    let result_index = if (op / 10i64.pow(num_params + 2)) % 10 == 2 {
      (relative_base as isize + instructions[ip + num_params as usize + 1] as isize) as usize
    } else {
      instructions[ip + num_params as usize + 1] as usize
    };

    let mut jumped = false;
    match main_op {
      1 => {
        instructions[result_index] = operands[0] + operands[1]
      },
      2 => {
        instructions[result_index] = operands[0] * operands[1]
      },
      3 => {
        let value = input.recv().unwrap();
        instructions[result_index] = value;
      },
      4 => {
        if let Err(_) = output.send(operands[0]) {
          // some sends might fail because the receiving end was already deallocated
        }
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
      9 => {
        relative_base += operands[0];
      }
      _ => panic!("At the math disco!")
    };

    if !jumped {
      ip += jump_table[main_op as usize];
    }
    op = instructions[ip];
  }

  println!("outputs: {:?}", outputs);

  if outputs.len() > 0 {
    outputs[outputs.len() - 1]
  } else {
    instructions[0]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn verify_203_works() {
    assert_eq!(isa_interpreter(&mut vec![109, 3, 203, 4, 4, 7, 99, 0], 1), 1);
  }
}