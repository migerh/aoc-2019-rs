use std::sync::Arc;
use std::thread::spawn;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;
use std::sync::RwLock;
use std::sync::mpsc::{channel, Receiver};
use crate::intcode::{parse_instructions, isa_interpreter_async};

struct Message {
    to: i64,
    x: i64,
    y: i64,
}

impl Message {
    fn new(to: i64, x: i64, y: i64) -> Self {
        Self { to, x, y }
    }
}

fn output_to_queue(recv: Receiver<i64>, queue: Arc<RwLock<VecDeque<Message>>>) {
    let mut buffer = vec![];

    loop {
        let value = recv.recv();
        if let Err(_) = value {
            // println!("Err in monitor thread while reading data: {}", err);
            break;
        }

        if let Ok(value) = value {
            buffer.push(value);

            if buffer.len() == 3 {
                let message = Message::new(buffer[0], buffer[1], buffer[2]);
                buffer = vec![];
                let mut queue = queue.write().unwrap();
                queue.push_back(message);
            }
        }
    }
}

fn switch(senders: Vec<Sender<i64>>, queue: Arc<RwLock<VecDeque<Message>>>) -> Message {
    // boot up the NICs
    for (i, s) in senders.iter().enumerate() {
        s.send(i as i64).unwrap();
    }

    loop {
        {
            let mut q = queue.write().unwrap();
            while let Some(m) = q.pop_front() {
                if m.to == 255 {
                    return m;
                }

                senders[m.to as usize].send(m.x).unwrap();
                senders[m.to as usize].send(m.y).unwrap();
            }
        }

        for s in &senders {
            s.send(-1).unwrap();
        }
    }
}

fn init(input: &str) -> (Vec<Sender<i64>>, Arc<RwLock<VecDeque<Message>>>) {
    let instructions = parse_instructions(input);
    let mut senders = vec![];
    let queue = Arc::new(RwLock::new(VecDeque::new()));
    let mut nics = vec![];
    let mut output_monitors = vec![];

    for _ in 0..50 {
        let (send, recv) = channel();
        let (msend, mrecv) = channel();
        senders.push(send);

        let q = queue.clone();
        let monitor_thread = spawn(move || output_to_queue(mrecv, q));
        output_monitors.push(monitor_thread);

        let instr = instructions.clone();
        let nic_thread = spawn(move || isa_interpreter_async(instr, recv, msend));
        nics.push(nic_thread);
    }

    (senders, queue.clone())
}

#[aoc(day23, part1)]
fn problem1(input: &str) -> i64 {
    let (senders, queue) = init(input);
    let switch_thread = spawn(move || switch(senders, queue));

    let result = switch_thread.join().unwrap();

    result.y
}

fn switch_v2(senders: Vec<Sender<i64>>, queue: Arc<RwLock<VecDeque<Message>>>) -> Message {
    // boot up the NICs
    for (i, s) in senders.iter().enumerate() {
        s.send(i as i64).unwrap();
    }

    let mut nat = Message::new(0, -1, -1);
    let mut idle_counter = 0;
    let mut last_sent = 0;

    loop {
        {
            let mut q = queue.write().unwrap();
            while let Some(m) = q.pop_front() {
                idle_counter = 0;
                if m.to == 255 {
                    nat = m;
                    continue;
                }

                senders[m.to as usize].send(m.x).unwrap();
                senders[m.to as usize].send(m.y).unwrap();
            }
        }

        for s in &senders {
            s.send(-1).unwrap();
        }
        idle_counter += 1;

        if idle_counter > 5 {
            idle_counter = 0;

            if last_sent == nat.y {
                return nat;
            }

            senders[0].send(nat.x).unwrap();
            senders[0].send(nat.y).unwrap();
            last_sent = nat.y;
        }
    }
}

#[aoc(day23, part2)]
fn problem2(input: &str) -> i64 {
    let (senders, queue) = init(input);
    let switch_thread = spawn(move || switch_v2(senders, queue));

    let result = switch_thread.join().unwrap();
    result.y
}