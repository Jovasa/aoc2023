use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, )]
enum Direction {
    X,
    Y,
    Z,
    CUBE,
}

fn main() {
    let input = std::fs::read_to_string("data/day22.txt").unwrap();

    // z, y, x
    let mut cube = vec![vec![vec![0u16; 10]; 10]; 350];

    let mut directions = HashMap::new();
    let mut xy_ranges = HashMap::new();

    let mut all_blocks = Vec::new();

    input.lines().enumerate().for_each(
        |(c, l)| {
            let mut t = l.split("~");
            let first = t.next().unwrap()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let second = t.next().unwrap()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let c = (c + 1) as u16;

            let direction = if first[0] != second[0] {
                Direction::X
            } else if first[1] != second[1] {
                Direction::Y
            } else if first[2] != second[2] {
                Direction::Z
            } else {
                Direction::CUBE
            };

            all_blocks.push(c);

            directions.insert(c, direction);
            xy_ranges.insert(c, ((first[0], second[0]), (first[1], second[1])));

            for x in first[0]..=second[0] {
                for y in first[1]..=second[1] {
                    for z in first[2]..=second[2] {
                        cube[z][y][x] = c;
                    }
                }
            }
        }
    );

    let directions = directions;

    let mut settled = false;
    while !settled {
        settled = true;
        for z in 2..cube.len() {
            for y in 0..cube[0].len() {
                for x in 0..cube[0][0].len() {
                    let block = cube[z][y][x];
                    if block == 0 {
                        continue;
                    }
                    let direction = directions.get(&block).unwrap();
                    let ((x1, x2), (y1, y2)) = xy_ranges.get(&block).unwrap();
                    let x1 = *x1;
                    let x2 = *x2;
                    let y1 = *y1;
                    let y2 = *y2;

                    if *direction == Direction::Z && cube[z - 1][y][x] == block {
                        continue;
                    }
                    let can_fall = {
                        let mut can_fall = true;
                        for y in y1..=y2 {
                            for x in x1..=x2 {
                                can_fall = can_fall && cube[z - 1][y][x] == 0;
                            }
                        }
                        can_fall
                    };
                    if can_fall {
                        settled = false;
                        for y in y1..=y2 {
                            for x in x1..=x2 {
                                cube[z - 1][y][x] = block;
                                cube[z][y][x] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut supported_by = HashMap::new();
    let mut supports = HashMap::new();
    for z in 1..cube.len() {
        for y in 0..cube[0].len() {
            for x in 0..cube[0][0].len() {
                let block = cube[z][y][x];
                if block == 0 {
                    continue;
                }
                let ((x1, x2), (y1, y2)) = xy_ranges.get(&block).unwrap();
                let x1 = *x1;
                let x2 = *x2;
                let y1 = *y1;
                let y2 = *y2;

                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let below = cube[z - 1][y][x];
                        if below != 0 && below != block {
                            supports.entry(below).or_insert_with(|| HashSet::new()).insert(block);
                            supported_by.entry(block).or_insert_with(|| HashSet::new()).insert(below);
                        }
                        let above = cube[z + 1][y][x];
                        if above != 0 && above != block {
                            supports.entry(block).or_insert_with(|| HashSet::new()).insert(above);
                            supported_by.entry(above).or_insert_with(|| HashSet::new()).insert(block);
                        }
                    }
                }
            }
        }
    }

    let mut can_be_removed = HashSet::new();
    for block in &all_blocks {
        if let None = supports.get(block) {
            can_be_removed.insert(*block);
            continue;
        }

        let mut cbr = true;
        for supported in supports.get(block).unwrap() {
            if !cbr {
                break;
            }
            cbr = cbr && (supported_by.get(supported).unwrap().len() > 1);
        }
        if cbr {
            can_be_removed.insert(*block);
        }
    }
    println!("{}", can_be_removed.len());

    let mut total = 0;
    for block in &all_blocks {
        let mut removed = HashSet::new();
        removed.insert(*block);
        count_concecutive_falls(*block, &supported_by, &supports, &mut removed);
        total += removed.len() - 1;
    }
    println!("{}", total);
}

fn count_concecutive_falls(
    c: u16,
    supported_by: &HashMap<u16, HashSet<u16>>,
    supports: &HashMap<u16, HashSet<u16>>,
    removed: &mut HashSet<u16>,
)  {
    if let None = supports.get(&c) {
        return;
    }
    let mut to_be_removed = HashSet::new();
    for supported in supports.get(&c).unwrap() {
        let supporters = supported_by.get(supported).unwrap();
        if supporters.iter().all(|x| removed.contains(x)) {
            removed.insert(*supported);
            to_be_removed.insert(*supported);
        }
    }
    for supported in &to_be_removed {
        count_concecutive_falls(*supported, supported_by, supports, removed);
    }
}