use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day5.txt").unwrap();
    let mut rows = input.lines();
    let seeds = rows
        .next().unwrap()
        .split(": ")
        .last().unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    println!("{:?}", seeds);

    let mut mapping: HashMap<String, Vec<Vec<u64>>> = HashMap::new();
    let mut from_to_map = HashMap::new();

    let mut from = "seed".to_owned();
    let mut to;
    for line in rows {
        if line.is_empty() {
            continue;
        }
        if line.contains(":") {
            let map_data = line.split(" ").next().unwrap().split("-to-").collect::<Vec<&str>>();
            from = map_data[0].to_owned();
            to = map_data[1].to_owned();
            from_to_map.insert(from.clone(), to.clone());
            mapping.insert(from.clone(), Vec::new());
        } else {
            let data = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            mapping.get_mut(&from).unwrap().push(data);
        }
    }
    let mut smallest = u64::MAX;
    for seed in seeds {
        let mut current = seed;
        let mut start = "seed".to_owned();
        while start != "location" {
            let data = mapping.get(&start).unwrap();
            let to = match from_to_map.get(&start) {
                Some(t) => t,
                None => break,
            };
            let mut next = 0;
            for d in data {
                if d[1] <= current && current <= d[1] + d[2]{
                    let offset = current - d[1];
                    next = d[0] + offset;
                    break;
                }
            }
            current = next;
            start = to.clone();
        }
        if current < smallest {
            smallest = current;
        }
    }
    println!("{:?}", smallest);
}