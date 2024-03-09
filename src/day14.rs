
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

type IterationFunc = fn(usize, usize) -> (usize, usize);

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

    tilt(&grid, height, width);
}

fn tilt(grid: &Vec<Vec<Position>>, height: usize, width: usize) {
    let mut total = 0;
    for x in 0..width {
        let mut last_rock = 0;
        for y in 0..height {
            match grid[y][x] {
                Position::RollingRock => {
                    last_rock += 1;
                    total += height - last_rock + 1;
                }
                Position::StationaryRock => {
                    last_rock = y + 1;
                }

                _ => {}
            }
        }
    }
    println!("{}", total);
}