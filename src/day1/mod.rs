use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn fuel_for_one(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_iterative(mass: i32) -> i32 {
    let mut fuel = fuel_for_one(mass);
    let mut total_fuel = fuel;

    while fuel > 0 {
        fuel = fuel_for_one(fuel);
        if fuel > 0 {
            total_fuel += fuel;
        }
    }

    total_fuel
}

fn fuel_for_delivery(input: &Vec<i32>, f: &dyn Fn(i32) -> i32) -> i32 {
    let total_fuel: i32 = input.iter().map(|m| f(*m)).sum();

    total_fuel
}

#[aoc(day1, part1)]
pub fn problem1(input: &Vec<i32>) -> i32 {
    fuel_for_delivery(input, &fuel_for_one)
}

#[aoc(day1, part2)]
pub fn problem2(input: &Vec<i32>) -> i32 {
    fuel_for_delivery(input, &fuel_iterative)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterative_fuel_example_1() {
        assert_eq!(fuel_iterative(1969), 966);
    }

    #[test]
    fn iterative_fuel_example_2() {
        assert_eq!(fuel_iterative(100756), 50346);
    }
}
