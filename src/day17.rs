use std::cmp::Ordering;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, )]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
struct Path {
    x: i16,
    y: i16,
    direction: Direction,
    steps_in_direction: u8,
    heat: u32,
}

impl Hash for Direction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Direction::North => 0.hash(state),
            Direction::South => 1.hash(state),
            Direction::East => 2.hash(state),
            Direction::West => 3.hash(state),
        }
    }
}

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.direction.hash(state);
        self.steps_in_direction.hash(state);
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.direction == other.direction && self.steps_in_direction == other.steps_in_direction
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl Eq for Path {}

impl Path {
    fn new(x: i16, y: i16, direction: Direction, steps_in_direction: u8, heat: u32) -> Self {
        Path { x, y, direction, steps_in_direction, heat }
    }
}


fn main() {
    let input = std::fs::read_to_string("data/day17.txt").unwrap();

    let grid = input.lines()
        .map(|x| x.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut states = std::collections::HashSet::new();
    states.insert(Path { x: 0, y: 0, direction: Direction::South, steps_in_direction: 0, heat: 0 });
    states.insert(Path { x: 0, y: 0, direction: Direction::East, steps_in_direction: 0, heat: 0 });

    let mut queue = std::collections::BinaryHeap::new();
    queue.push(Path { x: 0, y: 0, direction: Direction::South, steps_in_direction: 0, heat: 0 });
    queue.push(Path { x: 0, y: 0, direction: Direction::East, steps_in_direction: 0, heat: 0 });

    let mut smallest = std::u32::MAX;

    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        let x = path.x;
        let y = path.y;
        if x == width as i16 - 1 && y == height as i16 - 1 {
            smallest = std::cmp::min(smallest, path.heat);
            continue;
        }
        let direction = path.direction;
        let steps_in_direction = path.steps_in_direction;

        let heat = path.heat;

        let possible_steps = match direction {
            Direction::North => [
                Path::new(x, y - 1, Direction::North, steps_in_direction + 1, 0),
                Path::new(x + 1, y, Direction::East, 1, 0),
                Path::new(x - 1, y, Direction::West, 1, 0),
            ],
            Direction::South => [
                Path::new(x, y + 1, Direction::South, steps_in_direction + 1, 0),
                Path::new(x + 1, y, Direction::East, 1, 0),
                Path::new(x - 1, y, Direction::West, 1, 0),
            ],
            Direction::East => [
                Path::new(x + 1, y, Direction::East, steps_in_direction + 1, 0),
                Path::new(x, y + 1, Direction::South, 1, 0),
                Path::new(x, y - 1, Direction::North, 1, 0),
            ],
            Direction::West => [
                Path::new(x - 1, y, Direction::West, steps_in_direction + 1, 0),
                Path::new(x, y + 1, Direction::South, 1, 0),
                Path::new(x, y - 1, Direction::North, 1, 0),
            ],
        };
        for step in possible_steps.iter() {
            let next_x = step.x;
            let next_y = step.y;
            let next_direction = step.direction;
            let next_steps_in_direction = step.steps_in_direction;

            if next_direction == direction && next_steps_in_direction > 3 {
                continue;
            }

            if next_x < 0 || next_x >= width as i16|| next_y < 0 || next_y >= height as i16 {
                continue;
            }

            if states.contains(&step) {
                continue;
            }

            if grid[next_y as usize][next_x as usize] == 0 {
                continue;
            }
            let next_step = Path::new(next_x, next_y, next_direction, next_steps_in_direction,  heat + grid[next_y as usize][next_x as usize] as u32);

            states.insert(next_step);
            queue.push(next_step);
        }
    }
    println!("{}", smallest);

}
