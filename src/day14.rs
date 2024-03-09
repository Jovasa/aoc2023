use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Position {
    StationaryRock,
    RollingRock,
    Empty,
}

impl Position {
    fn from_char(c: char) -> Position {
        match c {
            '.' => Position::Empty,
            'O' => Position::RollingRock,
            '#' => Position::StationaryRock,
            _ => panic!()
        }
    }
}

fn grid_to_string(grid: &Vec<Vec<Position>>) -> String {
    grid.iter()
        .map(|x| x.iter()
            .map(|c| match c {
                Position::Empty => '.',
                Position::RollingRock => 'O',
                Position::StationaryRock => '#',
            })
            .collect::<String>()
        )
        .collect::<Vec<_>>()
        .join("\n")
}


fn calculate_load(grid: &Vec<Vec<Position>>) -> usize {
    let mut load = 0;
    let height = grid.len();
    for (y, row) in grid.iter().enumerate() {
        for position in row {
            if *position == Position::RollingRock {
                load += height - y;
            }
        }
    }
    load
}


fn main() {
    let input = std::fs::read_to_string("data/day14.txt").unwrap();

    let grid = input.lines()
        .map(|x| x.chars()
            .map(|c| Position::from_char(c))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut encountered_iterations = std::collections::HashMap::new();

    let mut grid = grid;
    for iteration in 0..1_000_000_000 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let new_grid_n = tilt(&grid, height, width, |s, f| (s, f));
        let new_grid_w = tilt(&new_grid_n, width, height, |s, f| (f, height - s - 1));
        let new_grid_s = tilt(&new_grid_w, height, width, |s, f| (height - s - 1, width - f - 1));
        let new_grid_e = tilt(&new_grid_s, width, height, |s, f| (width - f - 1, s));
        let grid_string = grid_to_string(&new_grid_e);
        grid = new_grid_e;
        grid_string.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(last_iteration) = encountered_iterations.get(&hash) {
            let cycle_length = iteration - last_iteration;
            println!("{}", cycle_length);
            let remaining_iterations = 1_000_000_000 - iteration - 1;
            let remaining_iterations = remaining_iterations % cycle_length;
            for _ in 0..remaining_iterations {
                let new_grid_n = tilt(&grid, height, width, |s, f| (s, f));
                let new_grid_w = tilt(&new_grid_n, width, height, |s, f| (f, height - s - 1));
                let new_grid_s = tilt(&new_grid_w, height, width, |s, f| (height - s - 1, width - f - 1));
                let new_grid_e = tilt(&new_grid_s, width, height, |s, f| (width - f - 1, s));
                grid = new_grid_e;
            }
            println!("{}", calculate_load(&grid));
            break;
        }

        encountered_iterations.insert(hash, iteration);
    }
}

fn tilt<F>(grid: &Vec<Vec<Position>>, inner_dimension: usize, outer_dimension: usize, iter_order_func: F) -> Vec<Vec<Position>>
     where F: Fn(usize, usize) -> (usize, usize)
{
    let mut new_grid = grid.clone();
    for s in 0..outer_dimension {
        let mut last_rock = (s, 0);
        for f in 0..inner_dimension {
            let (x, y) = iter_order_func(s, f);
            match grid[y][x] {
                Position::RollingRock => {
                    let (new_x, new_y) = iter_order_func(last_rock.0, last_rock.1);
                    // assert_eq!(new_grid[new_y][new_x], Position::Empty);
                    new_grid[y][x] = Position::Empty;
                    new_grid[new_y][new_x] = Position::RollingRock;
                    last_rock.1 += 1;
                }
                Position::StationaryRock => {
                    last_rock = (s, f + 1);
                }
                Position::Empty => {
                    new_grid[y][x] = Position::Empty;
                }
            }
        }
    }
    new_grid
}