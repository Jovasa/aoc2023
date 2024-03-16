use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("data/day21.txt").unwrap();

    let mut start = (0, 0);
    let mut count = 0;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)|
            l
                .chars()
                .enumerate()
                .map(
        |(x, c)| match c {
            '.' => 0,
            '#' => {
                count += 1;
                u16::MAX
            },
            'S' => {
                start = (x as i32, y as i32);
                0
            },
            x => panic!("Invalid character {x}"),
        }
    ).collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("{}", count);

    let mut walked_grid = grid.clone();
    walk(start, &mut walked_grid);

    let mut odd_total = 0;
    let mut even_total = 0;
    count_possibilities(&mut walked_grid, &mut odd_total, &mut even_total, 600);
    // println!("{}", total);

    let mut top_corner_grid = grid.clone();
    walk((65, 130), &mut top_corner_grid);
    let mut top_corner_odd_total = 0;
    let mut top_corner_even_total = 0;
    count_possibilities(&mut top_corner_grid, &mut top_corner_odd_total, &mut top_corner_even_total, 131);

    let mut bottom_corner_grid = grid.clone();
    walk((65, 0), &mut bottom_corner_grid);
    let mut bottom_corner_odd_total = 0;
    let mut bottom_corner_even_total = 0;
    count_possibilities(&mut bottom_corner_grid, &mut bottom_corner_odd_total, &mut bottom_corner_even_total, 131);

    let mut left_corner_grid = grid.clone();
    walk((130, 65), &mut left_corner_grid);
    let mut left_corner_odd_total = 0;
    let mut left_corner_even_total = 0;
    count_possibilities(&mut left_corner_grid, &mut left_corner_odd_total, &mut left_corner_even_total, 131);

    let mut right_corner_grid = grid.clone();
    walk((0, 65), &mut right_corner_grid);
    let mut right_corner_odd_total = 0;
    let mut right_corner_even_total = 0;
    count_possibilities(&mut right_corner_grid, &mut right_corner_odd_total, &mut right_corner_even_total, 131);

    let map_size = walked_grid.len();
    let grid_size = 26501365 / map_size - 1;

    let even_maps_in_grid = ((grid_size + 1) / 2 * 2).pow(2);
    let odd_maps_in_grid = (grid_size / 2 * 2 + 1).pow(2);
}

fn count_possibilities(walked_grid: &mut Vec<Vec<u16>>, odd_total: &mut i32, even_total: &mut i32, limit: u16) {
    for y in 0..walked_grid.len() {
        for x in 0..walked_grid[0].len() {
            if walked_grid[y][x] & 1 == 1 && walked_grid[y][x] > 0 && walked_grid[y][x] <= limit {
                *odd_total += 1;
                // print!("O");
            } else if walked_grid[y][x] > 0 && walked_grid[y][x] <= limit  {
                *even_total += 1;
            }
            // else if walked_grid[y][x] <= 10000 {
            //     print!(".");
            // }
            // else {
            //     print!("#");
            // }
        }
        // println!();
    }
}

fn walk(start: (i32, i32), walked_grid: &mut Vec<Vec<u16>>) {
    let mut steps = 0;
    let mut current = HashSet::new();
    current.insert(start);
    while !current.is_empty() {
        let mut new_current = HashSet::new();
        for (x, y) in current {
            walked_grid[y as usize][x as usize] = steps;
            if x > 0 && walked_grid[y as usize][x as usize - 1] == 0 {
                new_current.insert((x - 1, y));
            }
            if y > 0 && walked_grid[y as usize - 1][x as usize] == 0 {
                new_current.insert((x, y - 1));
            }
            if x < walked_grid[0].len() as i32 - 1 && walked_grid[y as usize][x as usize + 1] == 0 {
                new_current.insert((x + 1, y));
            }
            if y < walked_grid.len() as i32 - 1 && walked_grid[y as usize + 1][x as usize] == 0 {
                new_current.insert((x, y + 1));
            }
        }
        current = new_current;
        steps += 1;
    }
}