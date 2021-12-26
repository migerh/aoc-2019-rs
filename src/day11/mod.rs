use super::intcode::{isa_interpreter_async, parse_instructions};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

type Channel<T> = (Sender<T>, Receiver<T>);
type Coords = (i64, i64);

#[aoc_generator(day11)]
pub fn load(input: &str) -> Vec<i64> {
    parse_instructions(input)
}

fn robo_brain((send, recv): Channel<i64>, map: HashMap<Coords, i64>) -> HashMap<Coords, i64> {
    let mut position = (0, 0);
    let mut direction = (0, -1);
    let mut map = map;

    loop {
        let color = map.entry(position).or_insert(0i64);
        send.send(*color).unwrap();

        let new_color = recv.recv().unwrap();
        if new_color == 99 {
            break;
        }
        let rotation = recv.recv().unwrap();

        if rotation == 99 {
            break;
        }

        *color = new_color;
        direction = match (direction, rotation) {
            ((0, -1), 0) => (-1, 0),
            ((0, 1), 0) => (1, 0),
            ((0, -1), 1) => (1, 0),
            ((0, 1), 1) => (-1, 0),
            ((1, 0), 0) => (0, -1),
            ((-1, 0), 0) => (0, 1),
            ((1, 0), 1) => (0, 1),
            ((-1, 0), 1) => (0, -1),
            _ => panic!(
                "invalid direction/rotation combo: {:?}, {}",
                direction, rotation
            ),
        };
        position = (position.0 + direction.0, position.1 + direction.1);
    }

    map
}

#[aoc(day11, part1)]
pub fn problem1(instructions: &Vec<i64>) -> usize {
    let instructions = instructions.clone();
    let (robo_send, robo_recv) = channel();
    let (isa_send, isa_recv) = channel();

    let map = HashMap::new();
    let robo_thread = std::thread::spawn(move || robo_brain((isa_send, robo_recv), map));

    let robo_send_for_isa = robo_send.clone();
    let isa_thread = std::thread::spawn(move || {
        isa_interpreter_async(instructions, isa_recv, robo_send_for_isa)
    });

    isa_thread.join().unwrap();
    // terminate the robo brain thread
    robo_send.send(99).unwrap();
    robo_thread.join().unwrap().len()
}

#[aoc(day11, part2)]
pub fn problem2(instructions: &Vec<i64>) -> usize {
    let instructions = instructions.clone();

    println!("running 11-2");

    let (robo_send, robo_recv) = channel();
    let (isa_send, isa_recv) = channel();

    let mut map = HashMap::new();
    map.entry((0, 0)).or_insert(1i64);
    let robo_thread = std::thread::spawn(move || robo_brain((isa_send, robo_recv), map));

    let robo_send_for_isa = robo_send.clone();
    let isa_thread = std::thread::spawn(move || {
        isa_interpreter_async(instructions, isa_recv, robo_send_for_isa)
    });

    isa_thread.join().unwrap();
    // terminate the robo brain thread
    robo_send.send(99).unwrap();
    let result = robo_thread.join().unwrap();

    let mut mac = (0, 0);
    let mut mic = (0, 0);
    for (k, _) in result.iter() {
        mac = (max(mac.0, k.0), max(mac.1, k.1));
        mic = (min(mic.0, k.0), min(mic.1, k.1));
    }

    let mut canvas = vec![vec![' '; 50]; 6];
    for (k, v) in result.iter() {
        canvas[k.1 as usize][k.0 as usize] = if *v == 1 { 'X' } else { ' ' };
    }

    for line in canvas {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }

    0
}

#[cfg(test)]
mod test {
    #[test]
    fn problem1_example1() {
        assert_eq!(1, 1);
    }
}
