use std::fmt::Debug;

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    OPERATIONAL,
    UNKNOWN,
    DAMAGED,
}

impl State {
    fn from_char(s: &char) -> State {
        match s {
            '.' => State::OPERATIONAL,
            '?' => State::UNKNOWN,
            '#' => State::DAMAGED,
            _ => panic!("Invalid state")
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::OPERATIONAL => write!(f, "."),
            State::UNKNOWN => write!(f, "?"),
            State::DAMAGED => write!(f, "#"),
        }
    }
}

fn is_valid(lengths: &Vec<u32>, state: &Vec<State>) -> bool {
    let mut current_length = lengths.iter();
    let mut current_streak = 0;
    let mut found = 0;
    for s in state {
        match s {
            State::OPERATIONAL => {
                if current_streak != 0 {
                    if let Some(x) = current_length.next()   {
                        if current_streak != *x {
                            return false;
                        }
                    }
                    else {
                        return false;
                    }
                    found += 1;
                    current_streak = 0;
                }
            },
            State::UNKNOWN => {
                return false;
            },
            State::DAMAGED => {
                current_streak += 1;
            }

        }
    }
    match current_length.next() {
        Some(x) => *x == current_streak && found == lengths.len() - 1,
        None => current_streak == 0,
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<_>>();
        let pattern = parts[0];
        let lengths = parts[1].split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let state = pattern.chars().map(|x| State::from_char(&x)).collect::<Vec<_>>();
        let unknown_indexes = state.iter().enumerate().filter_map(|(i, x)| {
            if *x == State::UNKNOWN {
                Some(i)
            } else {
                None
            }
        }).collect::<Vec<_>>();

        let number_of_total_combinations = 2u32.pow(unknown_indexes.len() as u32);
        let mut valid_combinations = 0;
        for i in 0..number_of_total_combinations {
            let mut current_state = state.clone();
            for (j, index) in unknown_indexes.iter().enumerate() {
                if i & (1 << j) != 0 {
                    current_state[*index] = State::DAMAGED;
                } else {
                    current_state[*index] = State::OPERATIONAL;
                }
            }
            if is_valid(&lengths, &current_state) {
                valid_combinations += 1;
            }
        }
        total += valid_combinations;
    }

    println!("{}", total);
}