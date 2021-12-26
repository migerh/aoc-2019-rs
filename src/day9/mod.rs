use super::intcode::isa_interpreter;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .filter(|v| *v != "\n")
        .filter(|v| *v != "")
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn run_with_input(code: &Vec<i64>, input: i64) -> i64 {
    let mut instructions = code.clone();
    isa_interpreter(&mut instructions, input)
}

#[aoc(day9, part1)]
pub fn problem1(code: &Vec<i64>) -> i64 {
    run_with_input(code, 1)
}

#[aoc(day9, part2)]
pub fn problem2(code: &Vec<i64>) -> i64 {
    run_with_input(code, 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn problem1_example1() {
        let mut instructions = parse_input("1102,34915192,34915192,7,4,7,99,0");
        assert_eq!(isa_interpreter(&mut instructions, 1), 1219070632396864);
    }

    #[test]
    fn problem1_example2() {
        // let mut instructions = parse_input("108,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        // assert_eq!(isa_interpreter(&mut instructions, 1), 99);
    }

    #[test]
    fn problem1_example3() {
        let mut instructions = parse_input("104,1125899906842624,99");
        assert_eq!(isa_interpreter(&mut instructions, 1), 1125899906842624);
    }
}
