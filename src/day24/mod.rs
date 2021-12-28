use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Clone)]
struct Eris {
    map: Vec<Vec<char>>,
}

type Coords = (usize, usize);

impl Eris {
    fn new(map: Vec<Vec<char>>) -> Self {
        Self { map }
    }

    fn count_neighbors(&self, c: Coords) -> usize {
        let delta: Vec<isize> = vec![-1, 1];
        let (x, y) = c;
        let mut count = 0;

        for dy in &delta {
            let y = (y as isize + dy) as usize;

            if y < self.map.len() {
                if self.map[y][x] == '#' {
                    count += 1;
                }
            }
        }

        for dx in &delta {
            let x = (x as isize + dx) as usize;

            if x < self.map[y].len() {
                if self.map[y][x] == '#' {
                    count += 1;
                }
            }
        }

        count
    }

    fn tick(&mut self) {
        let mut new_map = vec![vec!['.'; 5]; 5];

        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                let neighbors = self.count_neighbors((x, y));

                new_map[y][x] = match (self.map[y][x], neighbors) {
                    ('#', 1) => '#',
                    ('.', 1) => '#',
                    ('.', 2) => '#',
                    _ => '.',
                }
            }
        }

        self.map = new_map;
    }

    fn rate(&self) -> usize {
        let flat = self.map.iter()
            .map(|v| v.iter())
            .flatten()
            .cloned()
            .collect::<Vec<_>>();

        flat.into_iter()
            .enumerate()
            .filter(|(_, v)| *v == '#')
            .map(|(i, _)| (2 as usize).pow(i as u32))
            .sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                print!("{}", self.map[y][x]);
            }
            println!("");
        }
    }
}

#[aoc_generator(day24)]
fn input_generator(input: &str) -> Eris {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Eris::new(map)
}

#[aoc(day24, part1)]
fn part1(eris: &Eris) -> usize {
    let mut eris = eris.clone();
    let mut ratings = HashSet::new();
    let mut current;

    loop {
        current = eris.rate();
        if ratings.contains(&current) {
            break;
        }
        ratings.insert(current);
        eris.tick();
    }

    current
}

#[derive(Clone)]
struct Eris2 {
    map: VecDeque<Vec<Vec<char>>>,
    center: usize,
}

type Coords2 = (usize, usize, usize);

impl Eris2 {
    fn new(initial_map: Vec<Vec<char>>) -> Self {
        let mut map = VecDeque::new();
        map.push_front(initial_map);
        let center = 0;
        Self { map, center }
    }

    fn get_row_bugs(&self, level: usize, row: usize) -> usize {
        if level >= self.map.len() {
            return 0
        }

        self.map[level][row].iter().filter(|v| **v == '#').count()
    }

    fn get_col_bugs(&self, level: usize, col: usize) -> usize {
        if level >= self.map.len() {
            return 0;
        }

        self.map[level].iter().map(|r| r[col]).filter(|v| *v == '#').count()
    }

    fn get_single_bug(&self, c: Coords2) -> usize {
        let (x, y, level) = c;
        if level >= self.map.len() {
            return 0;
        }

        if self.map[level][y][x] == '#' {
            1
        } else {
            0
        }
    }

    fn count_neighbors(&self, c: Coords2) -> usize {
        let delta: Vec<isize> = vec![-1, 1];
        let (x, y, l) = c;
        let mut count = 0;

        if x == 2 && y == 2 {
            return 0;
        }

        // consider tiles neighboring the next inner level
        if x == 1 && y == 2 {
            count += self.get_col_bugs(l + 1, 0);
        }

        if x == 3 && y == 2 {
            count += self.get_col_bugs(l + 1, 4);
        }

        if x == 2 && y == 1 {
            count += self.get_row_bugs(l + 1, 0);
        }

        if x == 2 && y == 3 {
            count += self.get_row_bugs(l + 1, 4);
        }

        // consider tiles neighboring the next outer level
        if x == 0 {
            if l > 0 {
                count += self.get_single_bug((1, 2, l-1));
            }
        }

        if x == 4 {
            if l > 0 {
                count += self.get_single_bug((3, 2, l-1));
            }
        }

        if y == 0 {
            if l > 0 {
                count += self.get_single_bug((2, 1, l-1));
            }
        }

        if y == 4 {
            if l > 0 {
                count += self.get_single_bug((2, 3, l-1));
            }
        }

        // count direct neighbors
        for dy in &delta {
            let y = (y as isize + dy) as usize;

            if y < self.map[l].len() {
                if self.map[l][y][x] == '#' {
                    count += 1;
                }
            }
        }

        for dx in &delta {
            let x = (x as isize + dx) as usize;

            if x < self.map[l][y].len() {
                if self.map[l][y][x] == '#' {
                    count += 1;
                }
            }
        }

        count
    }

    fn tick(&mut self) {
        let mut new_layer = vec![vec!['.'; 5]; 5];
        new_layer[2][2] = '?';
        self.map.push_front(new_layer.clone());
        self.map.push_back(new_layer.clone());
        self.center += 1;

        let mut new_map = VecDeque::new();
        for _ in 0..self.map.len() {
            new_map.push_back(new_layer.clone());
        }

        for l in 0..self.map.len() {
            for y in 0..self.map[l].len() {
                for x in 0..self.map[l][y].len() {
                    let neighbors = self.count_neighbors((x, y, l));

                    new_map[l][y][x] = match (self.map[l][y][x], neighbors) {
                        ('#', 1) => '#',
                        ('.', 1) => '#',
                        ('.', 2) => '#',
                        ('?', _) => '?',
                        _ => '.',
                    }
                }
            }
        }
        self.map = new_map;
    }

    fn rate(&self) -> usize {
        let flat = self.map.iter()
            .map(|v| v.iter().map(|r| r.iter()).flatten())
            .flatten()
            .cloned()
            .collect::<Vec<_>>();

        flat.into_iter()
            .enumerate()
            .filter(|(_, v)| *v == '#')
            .count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for l in 0..self.map.len() {
            println!("Depth {}:", (l as isize) - self.center as isize);
            for y in 0..self.map[l].len() {
                for x in 0..self.map[l][y].len() {
                    print!("{}", self.map[l][y][x]);
                }
                println!("");
            }
            println!("");
        }
    }
}

#[aoc(day24, part2)]
fn part2(eris: &Eris) -> usize {
    let mut eris = Eris2::new(eris.map.clone());

    for _ in 0..200 {
        eris.tick();
    }

    eris.rate()
}