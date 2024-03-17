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
    let mut to_from_map = HashMap::new();

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
            to_from_map.insert(to.clone(), from.clone());
            mapping.insert(from.clone(), Vec::new());
        } else {
            let data = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            mapping.get_mut(&from).unwrap().push(data);
        }
    }
    let mut smallest = u64::MAX;
    for seed in &seeds {
        let mut current = *seed;
        let mut start = "seed".to_owned();
        while start != "location" {
            let data = mapping.get(&start).unwrap();
            let to = match from_to_map.get(&start) {
                Some(t) => t,
                None => break,
            };
            let mut next = 0;
            for d in data {
                if d[1] <= current && current <= d[1] + d[2] {
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

    let mut ranges = Vec::new();
    // Take two concecutive seeds to build ranges
    let iter_end = seeds.len() / 2;
    for i in 0..iter_end {
        let start = seeds[i * 2];
        let end = seeds[i * 2 + 1];
        // for j in 0..end {
        //     ranges.push((start + j, 1));
        // }
        ranges.push((start, start + end));
    }

    let mut start = "seed".to_owned();
    let mut temp_ranges = ranges.clone();
    while start != "location" {
        let data = mapping.get(&start).unwrap();
        let to = match from_to_map.get(&start) {
            Some(t) => t,
            None => break,
        };
        let mut next = Vec::new();
        temp_ranges = ranges.clone();
        for d in data {
            let dest = d[0];
            let src = d[1];
            let sz = d[2];
            let src_end = src + sz;
            let mut inner_ranges = Vec::new();
            for (st, ed) in &temp_ranges {
                let before = (*st, (*ed).min(src));
                let inter = (src.max(*st), src_end.min(*ed));
                let after = (src_end.max(*st), *ed);

                if before.1 > before.0 {
                    inner_ranges.push(before);
                }
                if inter.1 > inter.0 {
                    next.push((inter.0 -src + dest, inter.1 - src + dest));
                }
                if after.1 > after.0 {
                    inner_ranges.push(after);
                }
            }
            temp_ranges = inner_ranges;
        }
        next.extend(temp_ranges.iter());
        ranges = next;
        if start == "seed" {
            println!("{} {:?}", to, ranges);
        }
        start = to.clone();
    }
    smallest = u64::MAX;
    for (s, _) in ranges {
        if s < smallest {
            smallest = s;
        }
    }
    println!("{:?}", smallest);
}