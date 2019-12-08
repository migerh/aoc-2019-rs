use std::sync::mpsc::{channel, Receiver, Sender};

pub fn isa_interpreter(instructions: &mut Vec<i32>, input: i32) -> i32 {
  isa_interpreter_mi(instructions, &vec![input])
}

pub fn isa_interpreter_mi(instructions: &mut Vec<i32>, input: &Vec<i32>) -> i32 {
  let (send, recv) = channel();
  for i in input {
    send.send(*i).unwrap();
  }
  isa_interpreter_mpsc(instructions, recv)
}

pub fn isa_interpreter_mpsc(instructions: &mut Vec<i32>, input: Receiver<i32>) -> i32 {
  let (send, _recv) = channel();
  isa_interpreter_async(instructions.clone(), input, send)
}

pub fn isa_interpreter_async(instructions: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) -> i32 {
  let mut ip = 0;
  let mut instructions = instructions.clone();
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

    let mut jumped = false;
    match main_op {
      1 => {
        instructions[result_index] = operands[0] + operands[1]
      },
      2 => {
        instructions[result_index] = operands[0] * operands[1]
      },
      3 => {
        instructions[result_index] = input.recv().expect("recv inside")
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
      _ => panic!("At the math disco!")
    };

    if !jumped {
      ip += jump_table[main_op as usize];
    }
    op = instructions[ip];
  }

  if outputs.len() > 0 {
    outputs[outputs.len() - 1]
  } else {
    instructions[0]
  }
}

