use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Size {
    GT,
    LT,
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(Hash)]
enum WorkOn {
    X,
    M,
    A,
    S,
}

struct Rule {
    work_on: Option<WorkOn>,
    size: Option<Size>,
    value: Option<u64>,
    target: String,
}

struct Workflow {
    rules: Vec<Rule>,
}


fn check_rules(current: &str, workflows: &HashMap<String, Workflow>, x: u64, m: u64, a: u64, s: u64) -> bool {
    if current == "A" {
        return true;
    } else if current == "R" {
        return false;
    }
    let current_workflow = workflows.get(current).unwrap();

    for rule in &current_workflow.rules {
        if rule.work_on.is_none() {
            return check_rules(&rule.target, workflows, x, m, a, s);
        }
        let value = match rule.work_on.unwrap() {
            WorkOn::X => x,
            WorkOn::M => m,
            WorkOn::A => a,
            WorkOn::S => s,
        };
        let passes = match rule.size {
            Some(Size::GT) => value > rule.value.unwrap(),
            Some(Size::LT) =>  value < rule.value.unwrap(),
            None => unreachable!(),
        };
        if passes {
            return check_rules(&rule.target, workflows, x, m, a, s);
        }
    }
    unreachable!()
}


fn divide_rules(current: &str, workflows: &HashMap<String, Workflow>,
                values: &HashMap<WorkOn, (u64, u64)> ) -> u64 {
    if values.values().any(|(x, m)| x > m) {
        return 0;
    }
    if current == "A" {
        return values.values().fold(1, |acc, (x, m)| acc * (m - x + 1));
    } else if current == "R" {
        return 0;
    }
    let current_workflow = workflows.get(current).unwrap();
    let mut total = 0;
    let mut my_values = values.clone();
    for rule in &current_workflow.rules {
        if rule.work_on.is_none() {
            return total + divide_rules(&rule.target, workflows, &my_values);
        }
        let value = match rule.work_on.unwrap() {
            WorkOn::X => my_values.get(&WorkOn::X).unwrap(),
            WorkOn::M => my_values.get(&WorkOn::M).unwrap(),
            WorkOn::A => my_values.get(&WorkOn::A).unwrap(),
            WorkOn::S => my_values.get(&WorkOn::S).unwrap(),
        };
        let new_values = my_values.clone();
        let mut new_fail_values = new_values.clone();
        let mut new_pass_values = new_values.clone();
        match rule.size.unwrap() {
            Size::GT => {
                new_fail_values.insert(rule.work_on.unwrap(), (value.0, rule.value.unwrap()));
                new_pass_values.insert(rule.work_on.unwrap(), (rule.value.unwrap() + 1, value.1));
            },
            Size::LT => {
                new_fail_values.insert(rule.work_on.unwrap(), (rule.value.unwrap(), value.1));
                new_pass_values.insert(rule.work_on.unwrap(), (value.0, rule.value.unwrap() - 1));
            },
        }
        total += divide_rules(&rule.target, workflows, &new_pass_values);
        my_values = new_fail_values;
    }
    unreachable!("Should not reach here");
}


fn main() {
    let input = std::fs::read_to_string("data/day19.txt").unwrap();

    let lines = input.lines();

    let workflows: HashMap<String, Workflow> = lines.clone().take_while(|x| x.len() != 0).map(|x| {
        let parts = x.split('{').collect::<Vec<_>>();
        let name = parts[0].trim();

        let rules = parts[1]
            .strip_suffix("}").unwrap()
            .split(",")
            .map(|rule| {
                if rule.len() <= 5 {
                    return Rule {
                        work_on: None,
                        size: None,
                        value: None,
                        target: rule.to_string(),
                    };
                }
                let mut c = rule.chars();
                let work_on = match c.next().unwrap() {
                    'x' => Some(WorkOn::X),
                    'm' => Some(WorkOn::M),
                    'a' => Some(WorkOn::A),
                    's' => Some(WorkOn::S),
                    _ => None
                };
                let size = match c.next().unwrap() {
                    '>' => Some(Size::GT),
                    '<' => Some(Size::LT),
                    _ => None
                };
                let value = c.clone()
                    .take_while(|x| x.is_digit(10))
                    .collect::<String>()
                    .parse::<u64>().ok();
                Rule {
                    work_on,
                    size,
                    value,
                    target: c
                        .collect::<String>()
                        .split(":")
                        .last()
                        .unwrap()
                        .to_string(),
                }
            }).collect::<Vec<_>>();
        (name.to_owned(), Workflow { rules })
    }).collect();

    // regex to match
    // {x=128,m=1590,a=606,s=53}
    let re = regex::Regex::new(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)}").unwrap();

    let mut total = 0;
    lines.skip(workflows.len() + 1).for_each(|x| {
        let caps = re.captures(x).unwrap();
        let x = caps.name("x").unwrap().as_str().parse::<u64>().unwrap();
        let m = caps.name("m").unwrap().as_str().parse::<u64>().unwrap();
        let a = caps.name("a").unwrap().as_str().parse::<u64>().unwrap();
        let s = caps.name("s").unwrap().as_str().parse::<u64>().unwrap();

        if check_rules("in", &workflows, x, m, a, s) {
            total += x + m + a + s;
        }
    });
    println!("{}", total);

    let mut values = HashMap::new();
    values.insert(WorkOn::X, (1, 4000));
    values.insert(WorkOn::M, (1, 4000));
    values.insert(WorkOn::A, (1, 4000));
    values.insert(WorkOn::S, (1, 4000));
    println!("{}", divide_rules("in", &workflows, &values));
}
