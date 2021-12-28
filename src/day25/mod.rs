use crate::intcode::isa_interpreter_async;
use std::thread::spawn;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::{Write, stdout, stdin};
use crate::intcode::parse_instructions;

fn is_waiting(buffer: &Vec<char>) -> bool {
    let expected = "Command?";
    let last = buffer.iter().rev().take(expected.len()).rev().collect::<String>();

    last == expected
}

fn flush(buffer: &Vec<char>) {
    let s = buffer.iter().collect::<String>();
    println!("{}", s);
    stdout().flush().expect("Flush did not work");
}

fn send_command(sender: &Sender<i64>, msg: &str) {
    for c in msg.chars() {
        sender.send(c as i64).unwrap();
    }
    sender.send('\n' as i64).unwrap();
}

#[allow(dead_code)]
fn ui(sender: Sender<i64>, receiver: Receiver<i64>) {
    let mut buffer = vec![];
    let mut s = String::new();

    loop {
        let rec = receiver.recv();

        if let Err(_) = rec {
            flush(&buffer);
            println!("ISA thread ended");
            break;
        }

        let rec = rec.unwrap() as u8 as char;
        buffer.push(rec);

        if is_waiting(&buffer) {
            flush(&buffer);
            buffer = vec![];
            stdin().read_line(&mut s).expect("Did not enter a string");

            if s.trim() == "quit" {
                break;
            }

            for c in s.trim().chars() {
                sender.send(c as i64).unwrap();
            }
            sender.send('\n' as i64).unwrap();
            s.clear();
        }
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Drain,
    Fill,
    Check,
}

fn auto(sender: Sender<i64>, receiver: Receiver<i64>) {
    let mut buffer = vec![];
    let commands = vec![
        "east",
        "take antenna",
        "west",
        "north",
        "take weather machine",
        "north",
        "take klein bottle",
        "east",
        "take spool of cat6",
        "east",
        "south",
        "take mug",
        "north",
        "north",
        "west",
        "north",
        "take cake",
        "south",
        "east",
        "east",
        "north",
        "north",
        "take tambourine",
        "south",
        "south",
        "south",
        "take shell",
        "north",
        "west",
        "south",
        "west",
        "south",
        "south",
    ];
    let mut next = 0;

    let mut state = State::Drain;
    let items = vec!["shell",
        "klein bottle",
        "tambourine",
        "weather machine",
        "spool of cat6",
        "mug",
        "cake"];
    let mut to_pack = 1;
    let mut inventory = items.clone();

    loop {
        let rec = receiver.recv();

        if let Err(_) = rec {
            flush(&buffer);
            println!("ISA thread ended");
            break;
        }

        let rec = rec.unwrap() as u8 as char;
        buffer.push(rec);

        if is_waiting(&buffer) {
            flush(&buffer);
            buffer = vec![];

            if next < commands.len() {
                send_command(&sender, commands[next]);
                next += 1;
                continue;
            }

            if state == State::Drain {
                if let Some(item) = inventory.pop() {
                    send_command(&sender, &format!("drop {}", item));
                } else {
                    state = State::Fill;
                }
            }

            if state == State::Fill {
                let mut take = Option::None;
                for i in 0..items.len() {
                    if ((1 << i) & to_pack != 0) && !inventory.contains(&items[i]) {
                        take = Some(items[i]);
                        break;
                    }
                }

                if let Some(i) = take {
                    inventory.push(i);
                    send_command(&sender, &format!("take {}", i));
                } else {
                    state = State::Check;
                }
            }

            if state == State::Check {
                send_command(&sender, "east");
                state = State::Drain;
                to_pack += 1;
            }
        }
    }
}

#[aoc(day25, part1)]
fn part1(input: &str) -> i64 {
    let instructions = parse_instructions(input);

    let (ui_send, ui_recv) = channel();
    let (isa_send, isa_recv) = channel();

    let ui_thread = spawn(move || auto(isa_send, ui_recv));
    let _isa_thread = spawn(move || isa_interpreter_async(instructions, isa_recv, ui_send));

    ui_thread.join().unwrap();

    0
}

#[aoc(day25, part2)]
fn part2(_input: &str) -> i64 {
    0
}