use std::ops::{BitAnd, Shl};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty = 0,
    NorthEnergized = 1,
    SouthEnergized = 2,
    WestEnergized = 4,
    EastEnergized = 8,
    VerticalSplitter = 16,
    HorizontalSplitter = 32,
    LeftMirror = 64,
    RightMirror = 128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}

fn is_energized(a: u8) -> bool {
    a & 0b1111 != 0
}

fn is_not_splitter_or_mirror(a: u8) -> bool {
    a & 0b11110000 == 0
}

fn count_number_energy(a: u8) -> usize {
    (a & 0xf).count_ones() as usize
}

impl Shl<Direction> for u8 {
    type Output = u8;

    fn shl(self, rhs: Direction) -> Self::Output {
        self << rhs as u8
    }
}

impl BitAnd<u8> for Tile {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self as u8 & rhs
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day16.txt").unwrap();

    let grid = input.lines()
        .map(|x| x.chars()
            .map(|c| match c {
                '.' => Tile::Empty,
                '|' => Tile::VerticalSplitter,
                '-' => Tile::HorizontalSplitter,
                '/' => Tile::LeftMirror,
                '\\' => Tile::RightMirror,
                _ => panic!()
            } as u8)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut possible_starts = vec![];

    for y in 0..height {
        possible_starts.push((0, y, Direction::East));
        possible_starts.push((width - 1, y, Direction::West));
    }
    for x in 0..width {
        possible_starts.push((x, 0, Direction::South));
        possible_starts.push((x, height - 1, Direction::North));
    }

    let mut maximum = 0;

    for start in possible_starts {
        let mut rays = vec![start];

        let mut grid = grid.clone();
        while !rays.is_empty() {
            let mut new_rays = Vec::new();
            for (x, y, direction) in &rays {
                let y = *y;
                let x = *x;
                let direction = *direction;
                let tile = grid[y][x];
                if tile & (1u8 << direction) != 0 {
                    continue;
                }
                grid[y][x] |= 1u8 << direction;
                if is_not_splitter_or_mirror(tile) {
                    add_new_ray(&mut grid, &mut new_rays, y, x, direction);
                    continue;
                }
                match direction {
                    Direction::East => {
                        if Tile::VerticalSplitter & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::North);
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::South);
                        } else if Tile::LeftMirror & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::North);
                        } else if Tile::RightMirror & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::South);
                        } else {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::East);
                        }
                    }
                    Direction::West => {
                        if Tile::VerticalSplitter & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::North);
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::South);
                        } else if Tile::LeftMirror & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::South);
                        } else if Tile::RightMirror & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::North);
                        } else {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::West);
                        }
                    }
                    Direction::North => {
                        if Tile::HorizontalSplitter & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::West);
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::East);
                        } else if Tile::LeftMirror & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::East);
                        } else if Tile::RightMirror & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::West);
                        } else {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::North);
                        }
                    }
                    Direction::South => {
                        if Tile::HorizontalSplitter & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::West);
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::East);
                        } else if Tile::LeftMirror & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::West);
                        } else if Tile::RightMirror & tile != 0 {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::East);
                        } else {
                            add_new_ray(&mut grid, &mut new_rays, y, x, Direction::South);
                        }
                    }
                }
            }
            rays = new_rays;
        }

        let result = grid.iter()
            .map(|x| x.iter()
                .filter(|c| is_energized(**c))
                .count()
            )
            .sum::<usize>();
        // println!("{}", result);
        maximum = maximum.max(result);
    }
    println!("{}", maximum);

}

fn add_new_ray(grid: &mut Vec<Vec<u8>>, new_rays: &mut Vec<(usize, usize, Direction)>, y: usize, x: usize, direction: Direction) -> bool {
    let (dx, dy) = match direction {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
        Direction::East => (1, 0),
    };
    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
    if nx < 0 || ny < 0 || ny as usize >= grid.len() || nx as usize >= grid[ny as usize].len() {
        return false;
    }
    new_rays.push((nx as usize, ny as usize, direction));
    true
}