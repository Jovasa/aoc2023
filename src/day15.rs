fn HASH(input: &str) -> u8 {
    let mut hash: u8 = 0;
    for c in input.chars() {
        hash = hash.overflowing_add(c as u8).0;
        hash = hash.overflowing_mul(17).0
    }
    hash
}


#[derive(Clone)]
struct Node {
    label: String,
    value: usize,
}


fn main() {
    let input = std::fs::read_to_string("data/day15.txt").unwrap().strip_suffix("\n").unwrap().to_owned();

    let parts = input.split(',').collect::<Vec<_>>();
    let result = parts.iter()
        .map(|x| HASH(x))
        .fold(0, |acc, x| acc + x as u32);
    println!("{}", result);

    let mut hashmap = Vec::new();
    for _ in 0..256 {
        hashmap.push(Vec::new())
    }

    for p in parts {
        if p.chars().last().unwrap() == '-' {
            let label = p.strip_suffix("-").unwrap().to_owned();
            let hash_value = HASH(&label);
            let all_items : &mut Vec<Node>= hashmap[hash_value as usize].as_mut();
            let mut found = all_items.len();
            for (i, item) in all_items.iter().enumerate() {
                if item.label == label {
                    found = i;
                    break;
                }
            }
            if found != all_items.len() {
                for i in found..all_items.len() - 1 {
                    all_items[i] = all_items[i + 1].clone();
                }
                all_items.pop();
            }
            continue
        }
        let value = p.chars().last().unwrap().to_digit(10).unwrap() as usize;
        let label = p.strip_suffix(value.to_string().as_str()).unwrap().strip_suffix("=").unwrap().to_owned();

        let hash_value = HASH(&label);
        let all_items: &mut Vec<Node> = hashmap[hash_value as usize].as_mut();
        let mut found = all_items.len();
        for (i, item) in all_items.iter().enumerate() {
            if item.label == label {
                found = i;
                break;
            }
        }
        if found != all_items.len() {
            all_items[found].value = value;
        } else {
            all_items.push(Node { label, value });
        }
    }

    let mut result = 0;
    for (i, items) in hashmap.iter().enumerate() {
        let box_value = i + 1;
        for (j, item) in items.iter().enumerate() {
            result += item.value * (j + 1) * box_value;
            // println!("{} {} {} {}", box_value, item.value, j + 1, item.value * (j + 1) * box_value);
        }
    }
    println!("{}", result);
}