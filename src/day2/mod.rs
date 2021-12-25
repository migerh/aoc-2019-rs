use super::intcode::isa_interpreter;
use std::num::ParseIntError;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input
        .split(",")
        .filter(|v| *v != "")
        .map(|v| v.parse::<i64>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn isa_interpreter_wrap(instructions: &mut Vec<i64>) -> i64 {
    isa_interpreter(instructions, 0)
}

fn patch_and_interpret(mut instructions: &mut Vec<i64>, noun: i64, verb: i64) -> i64 {
    instructions[1] = noun;
    instructions[2] = verb;

    isa_interpreter_wrap(&mut instructions)
}

fn patch_and_interpret_problem1(mut instructions: &mut Vec<i64>) -> i64 {
    instructions[1] = 12;
    instructions[2] = 2;

    isa_interpreter_wrap(&mut instructions)
}

#[aoc(day2, part1)]
pub fn problem1(instructions: &Vec<i64>) -> i64 {
    let mut instructions = instructions.clone();
    patch_and_interpret_problem1(&mut instructions)
}

#[aoc(day2, part2)]
pub fn problem2(instructions: &Vec<i64>) -> i64 {
    let expected_outcome: i64 = 19690720;
    let mut instructions = instructions.clone();
    let mut checksum = 0;

    for n in 0..100 {
        for v in 0..100 {
            let result = patch_and_interpret(&mut instructions, n, v);
            if result == expected_outcome {
                checksum = 100 * n + v;
            }
        }
    }

    checksum
}
