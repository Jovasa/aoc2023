use std::collections::HashSet;
use crate::PipePiece::{Ground, Horizontal, Vertical};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
enum PipePiece {
    Vertical = b'|',
    Horizontal = b'-',
    UpRightCorner = b'L',
    UpLeftCorner = b'J',
    DownRightCorner = b'F',
    DownLeftCorner = b'7',
    Ground = b'.',
    Outside = b'o',
}

impl From<u8> for PipePiece {
    fn from(c: u8) -> Self {
        match c {
            b'|' => PipePiece::Vertical,
            b'-' => PipePiece::Horizontal,
            b'L' => PipePiece::UpRightCorner,
            b'J' => PipePiece::UpLeftCorner,
            b'F' => PipePiece::DownRightCorner,
            b'7' => PipePiece::DownLeftCorner,
            b'.' => PipePiece::Ground,
            b'o' => PipePiece::Outside,
            _ => panic!("Invalid character"),
        }
    }

}
 impl PipePiece {
     fn as_char(&self) -> char {
         *self as u8 as char
     }
 }


fn main() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();
    let  lines = input.lines();

    let mut grid = Vec::new();
    let mut start = (1, 1);
    for (y, line) in lines.enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.bytes().enumerate() {
            if c == b'S' {
                start = (x, y);
                row.push(PipePiece::Ground);
                continue;
            }
            row.push(PipePiece::from(c));
        }
        grid.push(row);
    }
    let start = start;

    let mut visited = HashSet::new();
    visited.insert(start);
    let mut last_two = vec![];
    let mut possible_starts = HashSet::new();
    possible_starts.insert(PipePiece::Horizontal);
    possible_starts.insert(PipePiece::Vertical);
    possible_starts.insert(PipePiece::UpRightCorner);
    possible_starts.insert(PipePiece::UpLeftCorner);
    possible_starts.insert(PipePiece::DownRightCorner);
    possible_starts.insert(PipePiece::DownLeftCorner);

    if grid[start.1 - 1][start.0] == PipePiece::Vertical ||
        grid[start.1 - 1][start.0] == PipePiece::DownRightCorner ||
        grid[start.1 - 1][start.0] == PipePiece::DownLeftCorner {
        visited.insert((start.0, start.1 - 1));
        last_two.push((start.0, start.1 - 1));
        let mut starts_to_here = HashSet::new();
        starts_to_here.insert(PipePiece::Vertical);
        starts_to_here.insert(PipePiece::UpRightCorner);
        starts_to_here.insert(PipePiece::UpLeftCorner);
        possible_starts = possible_starts.intersection(&starts_to_here).cloned().collect();
    }
    if grid[start.1 + 1][start.0] == PipePiece::Vertical ||
        grid[start.1 + 1][start.0] == PipePiece::UpRightCorner ||
        grid[start.1 + 1][start.0] == PipePiece::UpLeftCorner {
        visited.insert((start.0, start.1 + 1));
        last_two.push((start.0, start.1 + 1));
        let mut starts_to_here = HashSet::new();
        starts_to_here.insert(PipePiece::Vertical);
        starts_to_here.insert(PipePiece::DownRightCorner);
        starts_to_here.insert(PipePiece::DownLeftCorner);
        possible_starts = possible_starts.intersection(&starts_to_here).cloned().collect();
    }
    if grid[start.1][start.0 - 1] == PipePiece::Horizontal ||
        grid[start.1][start.0 - 1] == PipePiece::DownRightCorner ||
        grid[start.1][start.0 - 1] == PipePiece::UpRightCorner {
        visited.insert((start.0 - 1, start.1));
        last_two.push((start.0 - 1, start.1));
        let mut starts_to_here = HashSet::new();
        starts_to_here.insert(PipePiece::Horizontal);
        starts_to_here.insert(PipePiece::UpLeftCorner);
        starts_to_here.insert(PipePiece::DownLeftCorner);
        possible_starts = possible_starts.intersection(&starts_to_here).cloned().collect();
    }
    if grid[start.1][start.0 + 1] == PipePiece::Horizontal ||
        grid[start.1][start.0 + 1] == PipePiece::DownLeftCorner ||
        grid[start.1][start.0 + 1] == PipePiece::UpLeftCorner {
        visited.insert((start.0 + 1, start.1));
        last_two.push((start.0 + 1, start.1));
        let mut starts_to_here = HashSet::new();
        starts_to_here.insert(PipePiece::Horizontal);
        starts_to_here.insert(PipePiece::UpRightCorner);
        starts_to_here.insert(PipePiece::DownRightCorner);
        possible_starts = possible_starts.intersection(&starts_to_here).cloned().collect();
    }
    assert_eq!(possible_starts.len(), 1);
    grid[start.1][start.0] = possible_starts.iter().next().unwrap().clone();

    assert_eq!(last_two.len(), 2);
    loop {
        let mut next_two = vec![];
        for &position in last_two.iter() {
            let (x, y) = position;
            match grid[y][x] {
                Vertical => {
                    if !visited.contains(&(x, y - 1)) {
                        visited.insert((x, y - 1));
                        next_two.push((x, y - 1));
                    }
                    if !visited.contains(&(x, y + 1)) {
                        visited.insert((x, y + 1));
                        next_two.push((x, y + 1));
                    }
                },
                PipePiece::Horizontal => {
                    if !visited.contains(&(x - 1, y)) {
                        visited.insert((x - 1, y));
                        next_two.push((x - 1, y));
                    }
                    if !visited.contains(&(x + 1, y)) {
                        visited.insert((x + 1, y));
                        next_two.push((x + 1, y));
                    }
                },
                PipePiece::UpRightCorner => {
                    if !visited.contains(&(x, y - 1)) {
                        visited.insert((x, y - 1));
                        next_two.push((x, y - 1));
                    }
                    if !visited.contains(&(x + 1, y)) {
                        visited.insert((x + 1, y));
                        next_two.push((x + 1, y));
                    }
                },
                PipePiece::UpLeftCorner => {
                    if !visited.contains(&(x, y - 1)) {
                        visited.insert((x, y - 1));
                        next_two.push((x, y - 1));
                    }
                    if !visited.contains(&(x - 1, y)) {
                        visited.insert((x - 1, y));
                        next_two.push((x - 1, y));
                    }
                },
                PipePiece::DownRightCorner => {
                    if !visited.contains(&(x, y + 1)) {
                        visited.insert((x, y + 1));
                        next_two.push((x, y + 1));
                    }
                    if !visited.contains(&(x + 1, y)) {
                        visited.insert((x + 1, y));
                        next_two.push((x + 1, y));
                    }
                },
                PipePiece::DownLeftCorner => {
                    if !visited.contains(&(x, y + 1)) {
                        visited.insert((x, y + 1));
                        next_two.push((x, y + 1));
                    }
                    if !visited.contains(&(x - 1, y)) {
                        visited.insert((x - 1, y));
                        next_two.push((x - 1, y));
                    }
                },
                _ => panic!("Invalid character")
            }
        }
        if next_two.is_empty() {
            break;
        }
        last_two = next_two;
    }
    println!("Visited: {}", visited.len() / 2);
    let mut up_sampled_grid = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, &c) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                new_row.push(c);
                if c == Horizontal || c == PipePiece::DownRightCorner || c == PipePiece::UpRightCorner {
                    new_row.push(Horizontal);
                }
                else {
                    new_row.push(Ground);
                }
            }
            else {
                new_row.push(Ground);
                new_row.push(Ground);
            }
        }
        up_sampled_grid.push(new_row);
        let mut new_row = Vec::new();
        for (x, &c) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                if c == Vertical || c == PipePiece::DownLeftCorner || c == PipePiece::DownRightCorner {
                    new_row.push(Vertical);
                }
                else {
                    new_row.push(Ground);
                }
                new_row.push(Ground);
            }
            else {
                new_row.push(Ground);
                new_row.push(Ground);
            }
        }
        up_sampled_grid.push(new_row);
    }

    let mut flood = vec![(0,0)];
    while !flood.is_empty() {
        let mut next_flood = vec![];
        for &position in flood.iter() {
            let (x, y) = position;
            if up_sampled_grid[y][x] == Ground {
                up_sampled_grid[y][x] = PipePiece::Outside;
                if x > 0 {
                    next_flood.push((x - 1, y));
                }
                if y > 0 {
                    next_flood.push((x, y - 1));
                }
                if x < up_sampled_grid[y].len() - 1 {
                    next_flood.push((x + 1, y));
                }
                if y < up_sampled_grid.len() - 1 {
                    next_flood.push((x, y + 1));
                }
            }
        }
        flood = next_flood;
    }

    let mut inside = 0;
    for row in up_sampled_grid.iter().step_by(2) {
        for &c in row.iter().step_by(2) {
            if c == PipePiece::Ground {
                inside += 1;
            }
        }
    }
    println!("Inside: {}", inside);
}