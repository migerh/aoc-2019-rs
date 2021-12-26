type Coords = (i64, i64);

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Coords> {
    let map = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut asteroids = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '#' {
                asteroids.push((x as i64, y as i64));
            }
        }
    }

    asteroids
}

fn distance(a: Coords, b: Coords) -> f32 {
    let dx = (a.0 - b.0) as f32;
    let dy = (a.1 - b.1) as f32;

    (dx * dx + dy * dy).sqrt()
}

fn is_on_line(a: Coords, b: Coords, c: Coords) -> bool {
    let dx1 = (b.0 - a.0) as f32;
    let dy1 = (b.1 - a.1) as f32;

    let dx2 = (c.0 - a.0) as f32;
    let dy2 = (c.1 - a.1) as f32;

    (dx2 * dy1 - dx1 * dy2).abs() < 1e-3
}

fn is_visible(map: &Vec<Coords>, asteroid: usize, other_asteroid: usize) -> bool {
    let coords_asteroid = map[asteroid];
    let coords_other = map[other_asteroid];

    let mut visible = true;
    for (i, coords) in map.iter().enumerate() {
        if i == asteroid || i == other_asteroid {
            continue;
        }

        // check if it is on the line
        let on_line = is_on_line(coords_asteroid, coords_other, *coords);

        // check if it is in between
        let is_between = (distance(coords_asteroid, *coords) + distance(coords_other, *coords)
            - distance(coords_asteroid, coords_other))
        .abs()
            < 1e-3;

        if on_line && is_between {
            visible = false;
            break;
        }
    }

    visible
}

fn count_visible_asteroids(map: &Vec<Coords>, asteroid: usize) -> usize {
    let mut count = 0;
    for (i, _) in map.iter().enumerate() {
        if i == asteroid {
            continue;
        }

        if is_visible(map, asteroid, i) {
            count += 1;
        }
    }

    count
}

fn find_best_asteroid(asteroids: &Vec<Coords>) -> i32 {
    let mut results = vec![];

    for (i, coords) in asteroids.iter().enumerate() {
        results.push((coords, count_visible_asteroids(&asteroids, i)));
    }

    results.sort_by(|a, b| b.1.cmp(&a.1));
    results[0].1 as i32
}

#[aoc(day10, part1)]
pub fn problem1(input: &Vec<Coords>) -> i32 {
    find_best_asteroid(input)
}

fn get_angle(a: Coords, c: Coords) -> f64 {
    let dx = (a.0 - c.0) as f64;
    let dy = (a.1 - c.1) as f64;
    let mut value = dy.atan2(dx);

    let pi = std::f64::consts::PI;
    let pi_half = pi / 2f64;
    if value > pi_half {
        value += -2f64 * pi;
    }

    if value > pi_half - 1e-3 {
        value = -2f64 * pi
    }

    value
}

fn get_all_angles(map: &Vec<Coords>, laser: Coords) -> Vec<f64> {
    let a = laser;
    let mut angles = vec![];
    for c in map {
        let value = get_angle(a, *c);
        angles.push(value);
    }
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mut i = 0;
    while i < angles.len() - 1 {
        if (angles[i] - angles[i + 1]).abs() < 1e-3 {
            angles.remove(i);
        } else {
            i += 1;
        }
    }

    angles
}

fn has_same_angle(angle: f64, asteroid: Coords, laser: Coords) -> bool {
    let other_angle = get_angle(laser, asteroid);
    (other_angle - angle).abs() < 1e-3
}

fn shoot(map: &Vec<Coords>, laser: Coords, angle: f64) -> Option<usize> {
    if map.is_empty() {
        return None;
    }

    let mut candidates = vec![];
    for (i, c) in map.iter().enumerate() {
        if has_same_angle(angle, *c, laser) {
            candidates.push((i, distance(laser, *c)));
        }
    }

    candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    if candidates.is_empty() {
        return None;
    }

    Some(candidates[0].0)
}

fn cleanup_angles(angles: Vec<f64>, empty: &Vec<f64>) -> Vec<f64> {
    let mut angles = angles;
    for e in empty {
        let pos = angles.iter().position(|v| v == e).unwrap();
        angles.remove(pos);
    }

    angles
}

fn destroy_asteroids(asteroids: &Vec<Coords>, laser: Coords) -> Vec<Coords> {
    let mut asteroids = asteroids.clone();
    let mut angles = get_all_angles(&asteroids, laser);

    let pos = asteroids.iter().position(|v| *v == laser).unwrap();
    asteroids.remove(pos);

    let mut destroyed_asteroids = vec![];
    let number_of_asteroids = asteroids.len();

    let mut count = 0;
    while destroyed_asteroids.len() < number_of_asteroids {
        count += 1;
        if count > 5 {
            break;
        }

        let mut empty_lasers = vec![];
        for angle in &angles {
            if let Some(index) = shoot(&asteroids, laser, *angle) {
                let shot_asteroid = asteroids.remove(index);
                destroyed_asteroids.push(shot_asteroid);
            } else {
                empty_lasers.push(*angle);
            }
        }

        angles = cleanup_angles(angles, &empty_lasers);
        empty_lasers.clear();
    }

    destroyed_asteroids
}

#[aoc(day10, part2)]
pub fn problem2(input: &Vec<Coords>) -> i64 {
    let destroyed_asteroids_in_order = destroy_asteroids(input, (13, 17));
    let d200 = destroyed_asteroids_in_order[199];
    d200.0 * 100 + d200.1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn problem1_example1() {
        let input = include_str!("./data/example1.txt");
        let input = parse_input(input);
        assert_eq!(find_best_asteroid(&input), 8);
    }

    #[test]
    fn problem2_example1() {
        let input = include_str!("./data/example1.txt");
        let input = parse_input(input);
        assert_eq!(destroy_asteroids(&input, (3, 4)).len(), 9);
    }

    #[test]
    fn problem2_example2() {
        let input = include_str!("./data/example3.txt");
        let input = parse_input(input);
        assert_eq!(destroy_asteroids(&input, (8, 3)).len(), 36);
    }

    #[test]
    fn isonline() {
        let a = (0, 2);
        let b = (0, 5);
        let c = (0, 3);

        assert_eq!(is_on_line(a, c, b), true);
    }
}
