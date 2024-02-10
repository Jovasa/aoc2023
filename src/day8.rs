use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();
    let mut lines = input.lines();

    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();

    let mut dir_map = std::collections::HashMap::new();

    // Match HVX = (SCS, XQN)
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let mut start_point = "AAA";
    let mut starts = Vec::new();

    for line in lines {
        let caps = re.captures(line).unwrap();
        let start = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();
        dir_map.insert(start, (left, right));
        if start.ends_with("A") {
            starts.push(start);
        }
    }

    let mut current = start_point;
    let mut num_steps = 0;
    while current != "ZZZ" {
        let (left, right) = dir_map.get(&current).unwrap();
        let dir = directions[num_steps % directions.len()];
        if dir == 'L' {
            current = left;
        } else {
            current = right;
        }
        num_steps += 1;
    }
    println!("num_steps: {}", num_steps);

    let mut current_positions = starts;
    let mut num_steps = 0;
    loop {
        let mut next_positions = Vec::new();
        let dir = directions[num_steps % directions.len()];
        for current in &current_positions {
            let (left, right) = dir_map.get(current).unwrap();
            if dir == 'L' {
                next_positions.push(left.to_owned());
            } else {
                next_positions.push(right.to_owned());
            }
        }
        if current_positions.iter().all(|x| x.ends_with("Z")) {
            println!("num_steps: {}", num_steps);
            return;
        }
        current_positions = next_positions;
        num_steps += 1;
    }
}