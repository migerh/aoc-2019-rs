use std::sync::mpsc::{channel, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use std::cmp::{min, max};
use super::intcode::{parse_instructions, isa_interpreter_async};

type Coords = (i64, i64);

fn render_thread(recv: Receiver<i64>) -> HashMap<Coords, u8> {
  let mut map = HashMap::new();
  loop {
    let x = recv.recv().unwrap();
    let y = recv.recv().unwrap();
    let tile = recv.recv().unwrap();

    if x == -1 && y == 0 {
      // todo
    }

    if tile == 99 {
      break;
    }

    map.entry((x, y)).or_insert(tile as u8);
  }

  map
}

fn render_map(map: &HashMap<Coords, u8>) {
  let mut mac = (0, 0);
  let mut mic = (0, 0);
  for (k, _) in map.iter() {
    mac = (max(mac.0, k.0), max(mac.1, k.1));
    mic = (min(mic.0, k.0), min(mic.1, k.1));
  }

  let mut canvas = vec![vec![' '; mac.0 as usize + 1]; mac.1 as usize + 1];
  let mut counter = 0;
  for (k, v) in map.iter() {
    canvas[k.1 as usize][k.0 as usize] = match *v {
      0 => ' ',
      1 => 'W',
      2 => 'B',
      3 => '-',
      4 => 'O',
      _ => 'X',
    };

    if *v == 2 {
      counter += 1;
    }
  }

  for line in canvas {
    for c in line {
      print!("{}", c);
    }
    println!("");
  }

  println!("result 13-1: {}", counter);
}

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let instructions = parse_instructions(&input);

  let (_isa_send, isa_recv) = channel();
  let (render_send, render_recv) = channel();
  let render_exit_send = render_send.clone();
  let game_thread = thread::spawn(move || {
    isa_interpreter_async(instructions, isa_recv, render_send)
  });
  let render_thread = thread::spawn(move || {
    render_thread(render_recv)
  });

  game_thread.join().unwrap();
  println!("game thread closed");
  // exit render thread
  render_exit_send.send(0).unwrap();
  render_exit_send.send(0).unwrap();
  render_exit_send.send(99).unwrap();

  let map = render_thread.join().unwrap();

  println!("map: {:?}", map);
  render_map(&map);
}

fn send_command(send: &Sender<i64>, dx: i64) {
  if let Err(err) = send.send(dx) {
    println!("Error occurred while sending: {}", err);
  }
}

fn play(recv: Receiver<i64>, send: Sender<i64>) -> (HashMap<Coords, u8>, i64) {
  let mut map = HashMap::new();
  let mut ball = (0, 0);
  let mut paddle = (0, 0);
  let mut paddle_initialized = false;
  let mut highscore = 0;
  let mut dx = 0;

  send_command(&send, dx);
  loop {
    let x = recv.recv().unwrap();
    let y = recv.recv().unwrap();
    let tile = recv.recv().unwrap();

    if x == -1 && y == 0 {
      highscore = tile;
      continue;
    }

    *map.entry((x, y))
      .or_insert(tile as u8) = tile as u8;

    if tile == 4 {
      ball = (x, y);
      dx = -(paddle.0 - ball.0).signum();
      if paddle_initialized {
        send_command(&send, dx);
        render_map(&map);
      }
    }

    if tile == 3 {
      paddle_initialized = true;
      paddle = (x, y);
      dx = -(paddle.0 - ball.0).signum();
    }

    if tile == 99 {
      break;
    }
  }

  (map, highscore)
}

pub fn problem2() {
  let input = include_str!("./data/input-1.txt");
  let mut instructions = parse_instructions(&input);
  // insert coin
  instructions[0] = 2;

  let (isa_send, isa_recv) = channel();
  let (render_send, render_recv) = channel();
  let play_exit_send = render_send.clone();
  let game_thread = thread::spawn(move || {
    isa_interpreter_async(instructions, isa_recv, render_send)
  });
  let play_thread = thread::spawn(move || {
    play(render_recv, isa_send)
  });

  game_thread.join().unwrap();
  println!("game thread closed");
  // exit render thread
  play_exit_send.send(0).unwrap();
  play_exit_send.send(0).unwrap();
  play_exit_send.send(99).unwrap();

  let (map, highscore) = play_thread.join().unwrap();

  println!("Highscore: {}", highscore);
  render_map(&map);
}