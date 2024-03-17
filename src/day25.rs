use std::collections::{HashMap, HashSet};
use std::fs;
use rand::{Rng, thread_rng};

fn kargers_algorithm(vertexes: Vec<(String, String)>) -> Option<usize> {
    let mut vertexes = vertexes.clone();
    let mut rng = thread_rng();
    let mut nodes = HashSet::new();
    for (a, b) in &vertexes {
        nodes.insert(a.clone());
        nodes.insert(b.clone());
    }
    while nodes.len() > 2 {
        let index = rng.gen_range(0..vertexes.len());
        let (a, b) = vertexes.remove(index);
        let ab = format!("{}-{}", a, b);
        assert!(a < b);
        vertexes = vertexes
            .iter()
            .map(|(x, y)| {
                if (x == &a && y == &b) || (x == &b && y == &a) {
                    return (x.clone(), y.clone());
                } else if x == &b || x == &a{
                    if &ab < y {
                        (ab.clone(), y.clone())
                    } else {
                        (y.clone(), ab.clone())
                    }
                } else if y == &b || y == &a{
                    if &ab < x {
                        (ab.clone(), x.clone())
                    } else {
                        (x.clone(), ab.clone())
                    }
                } else {
                    (x.clone(), y.clone())
                }
            })
            .collect();
        vertexes.retain(|(x, y)| x != &a || y != &b);
        nodes.remove(&b);
        nodes.remove(&a);
        nodes.insert(ab);
    }
    if vertexes.len() != 3 {
        return None;
    }
    Some(nodes.iter().fold(1, |acc, x| acc * (x.len() + 1 ) / 4))
}

fn main() {
    let input = fs::read_to_string("data/day25.txt").expect("File not found!");

    let mut connections = HashMap::new();
    let mut vertexes = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(": ");
        let from = parts.next().unwrap().to_string();
        parts.next().unwrap().split(" ").for_each(|x| {
            let to = x.split(",").next().unwrap().to_string();
            connections.entry(from.clone()).or_insert(Vec::new()).push(to.clone());
            connections.entry(to.clone()).or_insert(Vec::new()).push(from.clone());
            if to < from {
                vertexes.push((to.clone(), from.clone()));
            } else {
                vertexes.push((from.clone(), to.clone()));
            }
        });
    }

    loop {
        if let Some(result) = kargers_algorithm(vertexes.clone()) {
            println!("{}", result);
            break;
        }
    }
}