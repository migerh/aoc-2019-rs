#[aoc_generator(day4)]
pub fn input_generator(_input: &str) -> Vec<char> {
    vec![]
}

fn is_valid(p: u32) -> (bool, bool) {
    let mut has_double = false;
    let mut last_digit = 0;
    let mut group_sizes = vec![1];

    for i in 0..6 {
        let digit = p / 10u32.pow(5 - i) % 10;
        if last_digit > digit {
            return (false, false);
        }
        if last_digit == digit {
            has_double = true;
            if let Some(v) = group_sizes.last_mut() {
                *v += 1;
            }
        } else {
            group_sizes.push(1);
        }
        last_digit = digit;
    }

    let valid_for_2 = group_sizes.contains(&2);

    (has_double, valid_for_2)
}

#[aoc(day4, part1)]
pub fn problem1(_: &Vec<char>) -> u32 {
    let start = 147981;
    let end = 691423 + 1;

    let mut count = 0;

    for i in start..end {
        if is_valid(i).0 {
            count += 1;
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn problem2(_: &Vec<char>) -> u32 {
    let start = 147981;
    let end = 691423 + 1;

    let mut count = 0;

    for i in start..end {
        if is_valid(i).1 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn problem1_ex1() {
        assert_eq!(is_valid(111111u32), (true, false));
    }

    #[test]
    pub fn problem1_ex2() {
        assert_eq!(is_valid(223450u32), (false, false));
    }

    #[test]
    pub fn problem1_ex3() {
        assert_eq!(is_valid(123789u32), (false, false));
    }
    #[test]
    pub fn problem2_ex3() {
        assert_eq!(is_valid(111122u32), (true, true));
    }
}
