use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn isa_interpreter(instructions: &mut Vec<i32>, input: Receiver<i32>) -> i32 {
  let (send, _recv) = channel();
  isa_interpreter_async(instructions.clone(), input, send)
}

fn isa_interpreter_async(instructions: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) -> i32 {
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

  outputs[outputs.len() - 1]
}

fn thruster_output(program: &Vec<i32>, phases: &Vec<i32>) -> i32 {
  let mut input = 0;
  for phase in phases {
    let mut code = program.clone();
    let (send, recv) = channel();
    send.send(*phase).unwrap();
    send.send(input).unwrap();
    input = isa_interpreter(&mut code, recv);
  }

  input
}

fn load_input() -> Vec<i32> {
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
  let opcodes = load_input();
  let mut top = 0;
  for a in 0..5 {
    for b in 0..5 {
      if b == a {
        continue;
      }
      for c in 0..5 {
        if c == a || c == b {
          continue;
        }
        for d in 0..5 {
          if d == a || d == b || d == c {
            continue;
          }
          for e in 0..5 {
            if e == d || e == b || e == c || e == a {
              continue;
            }
            let result = thruster_output(&opcodes, &vec![a, b, c, d, e]);
            top = std::cmp::max(top, result);
          }
        }
      }
    }
  }

  println!("result: {}", top);
  top
}

fn thruster_feedback_loop(program: &Vec<i32>, phases: &Vec<i32>) -> i32 {
  let (send_a, recv_a) = channel();
  send_a.send(phases[0]).unwrap();
  send_a.send(0).unwrap();

  let (send_b, recv_b) = channel();
  send_b.send(phases[1]).unwrap();

  let (send_c, recv_c) = channel();
  send_c.send(phases[2]).unwrap();

  let (send_d, recv_d) = channel();
  send_d.send(phases[3]).unwrap();

  let (send_e, recv_e) = channel();
  send_e.send(phases[4]).unwrap();


  let mut threads = vec![];
  let code_a = program.clone();
  threads.push(thread::spawn(move || {
    isa_interpreter_async(code_a, recv_a, send_b)
  }));

  let code_b = program.clone();
  threads.push(thread::spawn(move || {
    isa_interpreter_async(code_b, recv_b, send_c)
  }));

  let code_c = program.clone();
  threads.push(thread::spawn(move || {
    isa_interpreter_async(code_c, recv_c, send_d)
  }));

  let code_d = program.clone();
  threads.push(thread::spawn(move || {
    isa_interpreter_async(code_d, recv_d, send_e)
  }));

  let code_e = program.clone();
  threads.push(thread::spawn(move || {
    isa_interpreter_async(code_e, recv_e, send_a)
  }));

  let mut results = vec![];
  for thread in threads {
    results.push(thread.join().unwrap());
  }

  *results.last().unwrap()
}

pub fn problem2() -> i32 {
  let opcodes = load_input();
  let mut top = 0;
  for a in 5..10 {
    for b in 5..10 {
      if b == a {
        continue;
      }
      for c in 5..10 {
        if c == a || c == b {
          continue;
        }
        for d in 5..10 {
          if d == a || d == b || d == c {
            continue;
          }
          for e in 5..10 {
            if e == d || e == b || e == c || e == a {
              continue;
            }
            let result = thruster_feedback_loop(&opcodes, &vec![a, b, c, d, e]);
            top = std::cmp::max(top, result);
          }
        }
      }
    }
  }

  println!("result: {}", top);
  0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem1_example1() {
    let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    assert_eq!(thruster_output(&program, &vec![4, 3, 2, 1, 0]), 43210);
  }

  #[test]
  fn problem1_example2() {
    let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
    101,5,23,23,1,24,23,23,4,23,99,0,0];
    assert_eq!(thruster_output(&program, &vec![0, 1, 2, 3, 4]), 54321);
  }

  #[test]
  fn problem1_example3() {
    let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
    1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    assert_eq!(thruster_output(&program, &vec![1,0,4,3,2]), 65210);
  }

  #[test]
  fn problem2_example1() {
    let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
    27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
    assert_eq!(thruster_feedback_loop(&program, &vec![9,8,7,6,5]), 139629729);
  }

  #[test]
  fn problem2_example2() {
    let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
      -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
      53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
    assert_eq!(thruster_feedback_loop(&program, &vec![9,7,8,5,6]), 18216);
  }
}