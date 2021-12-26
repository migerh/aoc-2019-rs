use std::sync::mpsc::{channel, Receiver};
use crate::intcode::{parse_instructions, isa_interpreter_async};

#[derive(Debug)]
enum Output {
    Crash(Vec<char>),
    Success(i64),
}

fn monitor(recv: Receiver<i64>) -> Output {
    let mut output = vec![];
    let mut success = 0;
    loop {
        let value = recv.recv();
        if let Err(err) = value {
            println!("Err in monitor thread while reading data: {}", err);
            break;
        } else if let Ok(value) = value {
            if value < 256 {
                output.push(char::from(value as u8));
            } else {
                success = value;
            }
        }
    }

    if success > 0 {
        Output::Success(success)
    } else {
        Output::Crash(output)
    }
}

#[aoc(day21, part1)]
pub fn problem1(input: &str) -> i64 {
    let instructions = parse_instructions(input);
    let sprintcode = "NOT B J
NOT C T
OR T J
AND D J
NOT A T
OR T J
WALK\n";
    let (isa_send, isa_recv) = channel();
    let (mon_send, mon_recv) = channel();
    let thread_monitor = std::thread::spawn(move || monitor(mon_recv));
    let isa_thread =
        std::thread::spawn(move || isa_interpreter_async(instructions, isa_recv, mon_send));

    for c in sprintcode.chars() {
        isa_send.send(c as i64).unwrap();
    }

    let result = thread_monitor.join().unwrap();
    isa_thread.join().unwrap();

    let mut damage = 0;
    if let Output::Success(d) = result {
        damage = d;
    } else if let Output::Crash(map) = result {
        let map: String = map.iter().collect();
        println!("{}", map);
    }

    damage
}

#[aoc(day21, part2)]
pub fn problem2(input: &str) -> i64 {
    let instructions = parse_instructions(input);
    let sprintcode = "NOT B J
NOT C T
OR T J
AND D J
AND H J
NOT A T
OR T J
RUN\n";
    let (isa_send, isa_recv) = channel();
    let (mon_send, mon_recv) = channel();
    let thread_monitor = std::thread::spawn(move || monitor(mon_recv));
    let isa_thread =
        std::thread::spawn(move || isa_interpreter_async(instructions, isa_recv, mon_send));

    for c in sprintcode.chars() {
        isa_send.send(c as i64).unwrap();
    }

    let result = thread_monitor.join().unwrap();
    isa_thread.join().unwrap();

    let mut damage = 0;
    if let Output::Success(d) = result {
        damage = d;
    } else if let Output::Crash(map) = result {
        let map: String = map.iter().collect();
        println!("{}", map);
    }

    damage
}