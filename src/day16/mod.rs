fn parse_input(input: &str) -> Vec<i64> {
  input
    .chars()
    .filter(|&v| v != '\n')
    .map(|v| v.to_string().parse::<i64>().unwrap())
    .collect::<Vec<_>>()
}

fn repeat_pattern(index: usize) -> Vec<i64> {
  let mut result = vec![];
  let base_pattern = vec![0, 1, 0, -1];

  for entry in base_pattern {
    for _ in 0..index {
      result.push(entry);
    }
  }

  result
}

fn fold(signal: &Vec<i64>) -> Vec<i64> {
  let result = signal.iter()
    .enumerate()
    .map(|(level, _)| {
      let repeat = repeat_pattern(level + 1);
      let rlen = repeat.len();
      let mut out = 0;
      for (index, inp) in signal.iter().enumerate() {
        out += inp * repeat[(index + 1) % rlen];
      }

      out.abs() % 10
    })
    .collect::<Vec<_>>();

  result
}

fn fft(signal: Vec<i64>) -> Vec<i64> {
  let mut signal = signal;
  for _ in 0..100 {
    signal = fold(&signal);
  }

  signal
}

fn fold2(signal: &Vec<i64>) -> Vec<i64> {
  let mut sum: i64 = signal.iter().sum();
  let mut result = vec![0; signal.len()];

  for (index, _) in signal.iter().enumerate() {
    result[index] = sum.abs() % 10;
    sum -= signal[index];
  }

  result
}

fn fft2(signal: Vec<i64>) -> Vec<i64> {
  let mut signal = signal;
  for _ in 0..100 {
    signal = fold2(&signal);
  }

  signal
}

fn simple_hash(signal: &Vec<i64>, start: usize) -> i64 {
  let mut result = 0;
  for i in 0..8 {
    result += 10i64.pow(i) * signal[start + (7 - i as usize)];
  }

  result
}

pub fn problem1() {
  let input = include_str!("./data/input-1.txt");
  let signal = parse_input(&input);
  let cleaned_signal = fft(signal);

  let result = simple_hash(&cleaned_signal, 0);

  println!("Result 16-1: {}", result);
}

pub fn problem2() {
  let input = include_str!("./data/input-1.txt");
  let signal = parse_input(&input);

  let mut real_signal = vec![];
  for _ in 0..10_000 {
    real_signal.append(&mut signal.clone());
  }

  let offset = (simple_hash(&signal, 0) / 10) as usize;
  let relevant: Vec<_> = real_signal.drain(offset..).collect();
  let cleaned_signal = fft2(relevant);
  let result = simple_hash(&cleaned_signal, 0);

  println!("Result 16-2: {}", result);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn repeat_pattern_index2_has_correct_length() {
    assert_eq!(repeat_pattern(2).len(), 8);
  }

  #[test]
  fn repeat_pattern_index2_is_correct() {
    assert_eq!(repeat_pattern(2)[1], 0);
    assert_eq!(repeat_pattern(2)[2], 1);
    assert_eq!(repeat_pattern(2)[3], 1);
    assert_eq!(repeat_pattern(2)[4], 0);
  }

  #[test]
  fn problem1_example1() {
    let input = "80871224585914546619083218645595";
    let signal = parse_input(&input);
    let cleaned_signal = fft(signal);
    let result = simple_hash(&cleaned_signal, 0);
    assert_eq!(result, 24176176);
  }

  #[test]
  fn problem1_example2() {
    let input = "19617804207202209144916044189917";
    let signal = parse_input(&input);
    let cleaned_signal = fft(signal);
    let result = simple_hash(&cleaned_signal, 0);
    assert_eq!(result, 73745418);
  }

  #[test]
  fn problem1_example3() {
    let input = "69317163492948606335995924319873";
    let signal = parse_input(&input);
    let cleaned_signal = fft(signal);
    let result = simple_hash(&cleaned_signal, 0);
    assert_eq!(result, 52432133);
  }
}