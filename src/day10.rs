use std::collections::HashSet;
use crate::PipePiece::Vertical;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u8)]
enum PipePiece {
    Vertical = b'|',
    Horizontal = b'-',
    UpRightCorner = b'L',
    UpLeftCorner = b'J',
    DownRightCorner = b'F',
    DownLeftCorner = b'7',
    Ground = b'.',
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
            _ => panic!("Invalid character"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();
    let  lines = input.lines();

    let mut grid = Vec::new();
    let mut start = (0, 0);
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
    if grid[start.1 - 1][start.0] == PipePiece::Horizontal ||
        grid[start.1 - 1][start.0] == PipePiece::DownRightCorner ||
        grid[start.1 - 1][start.0] == PipePiece::DownLeftCorner {
        visited.insert((start.0, start.1 - 1));
        last_two.push((start.0, start.1 - 1));
    }
    if grid[start.1 + 1][start.0] == PipePiece::Horizontal ||
        grid[start.1 + 1][start.0] == PipePiece::UpRightCorner ||
        grid[start.1 + 1][start.0] == PipePiece::UpLeftCorner {
        visited.insert((start.0, start.1 + 1));
        last_two.push((start.0, start.1 + 1));
    }
    if grid[start.1][start.0 - 1] == PipePiece::Vertical ||
        grid[start.1][start.0 - 1] == PipePiece::DownRightCorner ||
        grid[start.1][start.0 - 1] == PipePiece::UpRightCorner {
        visited.insert((start.0 - 1, start.1));
        last_two.push((start.0 - 1, start.1));
    }
    if grid[start.1][start.0 + 1] == PipePiece::Vertical ||
        grid[start.1][start.0 + 1] == PipePiece::DownLeftCorner ||
        grid[start.1][start.0 + 1] == PipePiece::UpLeftCorner {
        visited.insert((start.0 + 1, start.1));
        last_two.push((start.0 + 1, start.1));
    }
    assert!(last_two.len() == 2);
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
}