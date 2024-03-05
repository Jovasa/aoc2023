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


fn recursive_count(lengths: &Vec<u32>, state: &Vec<State>, lengths_index: usize, state_index: usize) -> u64 {
    if lengths_index == lengths.len() {
        for i in state_index..state.len() {
            if state[i] == State::DAMAGED {
                return 0;
            }
        }
        return 1;
    }
    let length = lengths[lengths_index] as usize;
    let mut total = 0;
    for i in state_index..state.len() - length + 1 {
        if state[i] == State::OPERATIONAL {
            continue
        }
        for j in state_index..i {
            if state[j] == State::DAMAGED {
                return total;
            }
        }
        let mut valid = true;
        for j in 1..length {
            valid &= state[j + i] != State::OPERATIONAL;
        }
        if !valid {
            continue
        }
        if length + i != state.len() && state[length + i] == State::DAMAGED {
            continue
        }
        total += recursive_count(lengths, state, lengths_index + 1, i + length + 1);
    }
    total
}


fn main() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<_>>();
        let pattern = parts[0];
        let lengths = parts[1].split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let state = pattern.chars().map(|x| State::from_char(&x)).collect::<Vec<_>>();

        let a = recursive_count(&lengths, &state, 0, 0);
        total += a;
    }

    println!("{}", total);
}