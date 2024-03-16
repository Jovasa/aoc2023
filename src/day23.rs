use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    SlopeDown,
    SlopeUp,
    SlopeLeft,
    SlopeRight,
}

fn main() {
    let input = std::fs::read_to_string("data/day23.txt").unwrap();

    let map = input.lines()
        .map(|l| l.chars()
            .map(|c| match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'v' => Tile::SlopeDown,
                '^' => Tile::SlopeUp,
                '<' => Tile::SlopeLeft,
                '>' => Tile::SlopeRight,
                _ => panic!("Invalid character"),
            }).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let already_visited = HashSet::new();

    let total = walk((0, 1), (0, 0), &map, &already_visited, 0);
    println!("{}", total);
}

fn can_go_to_tile(current: (usize, usize), previous: (usize, usize), tile: Tile) -> bool {
    return tile != Tile::Wall;
    match tile {
        Tile::Empty => true,
        Tile::Wall => false,
        Tile::SlopeDown => {
            if current.0 > previous.0 {
                true
            } else {
                false
            }
        }
        Tile::SlopeUp => {
            if current.0 < previous.0 {
                true
            } else {
                false
            }
        }
        Tile::SlopeLeft => {
            if current.1 < previous.1 {
                true
            } else {
                false
            }
        }
        Tile::SlopeRight => {
            if current.1 > previous.1 {
                true
            } else {
                false
            }
        }
    }
}

fn walk(current: (usize, usize),
        previous: (usize, usize),
        map: &Vec<Vec<Tile>>, already_visited:
        &HashSet<(usize, usize)>,
        call_stack: u64) -> usize {
    let mut current = current;
    let mut previous = previous;
    let mut total_steps = 0;
    println!("{:?}", call_stack);
    let mut already_visited = already_visited.clone();
    loop {
        if current.0 == map[0].len() - 1 && current.1 == map.len() - 2 {
            return total_steps;
        }
        already_visited.insert(current);
        let mut valid_next = Vec::new();
        total_steps += 1;
        let south = (current.0.overflowing_sub(1).0, current.1);
        if current.0 > 0 && can_go_to_tile(south, previous, map[current.0 - 1][current.1]) {
            if !already_visited.contains(&south) {
                valid_next.push(south);
            }
        }
        let north = (current.0 + 1, current.1);
        if current.0 < map.len() - 1 && can_go_to_tile(north, previous, map[current.0 + 1][current.1]) {
            if !already_visited.contains(&north) {
                valid_next.push(north);
            }
        }
        let west = (current.0, current.1.overflowing_sub(1).0);
        if current.1 > 0 && can_go_to_tile(west, previous, map[current.0][current.1 - 1]) {
            if !already_visited.contains(&west) {
                valid_next.push(west);
            }
        }
        let east = (current.0, current.1 + 1);
        if current.1 < map[0].len() - 1 && can_go_to_tile(east, previous, map[current.0][current.1 + 1]) {
            if !already_visited.contains(&east) {
                valid_next.push(east);
            }
        }
        if valid_next.len() == 0 {
            return 0;
        }
        if valid_next.len() == 1 {
            previous = current;
            current = valid_next[0];
        } else {
            let mut longest = 0;
            for next in valid_next {
                let already_visited = already_visited.clone();
                let length = walk(next, current, map, &already_visited, call_stack + 1);
                if length > longest {
                    longest = length;
                }
            }
            return total_steps + longest;
        }
    }
}