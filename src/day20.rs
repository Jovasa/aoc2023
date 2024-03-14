use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

/*
pub trait PulseSource {
    fn pulse<T>(&mut self, pulse: Pulse, target: &mut HashMap<String, T>)
    where T: PulseSource;
}

struct FlipFlop {
    state: bool,
    targets: Vec<String>,
}

struct Conjunction {
    memory: Pulse,
    targets: Vec<String>,
}

struct Broadcaster {
    targets: Vec<String>,
}

impl  FlipFlop {
    fn new(targets: Vec<String>) -> Self {
        Self {
            state: false,
            targets,
        }
    }
}

impl PulseSource for FlipFlop {

    fn pulse<T>(&mut self, pulse: Pulse, targets: &mut HashMap<String, T>)
    where T: PulseSource {
        if pulse == Pulse::High {
            return;
        }
        self.state = !self.state;
        let out = match self.state {
            true => Pulse::High,
            false => Pulse::Low,
        };
        for target in &self.targets {
            targets.get(target).unwrap().pulse(out, &mut targets);
        }
    }

}*/

fn main() {
    let input = std::fs::read_to_string("data/day20.txt").unwrap();

    let mut end_conditions = HashMap::new();
    end_conditions.insert("km".to_owned(), (0, 0));
    end_conditions.insert("kz".to_owned(), (0, 0));
    end_conditions.insert("qs".to_owned(), (0, 0));
    end_conditions.insert("xj".to_owned(), (0, 0));

    let parts = input.lines()
        .map(
            |x| {
                let mut p = x.split(" -> ");
                let first = p.next().unwrap();
                let rest = p.next().unwrap();
                let part_type = match first.chars().next().unwrap() {
                    '%' => Module::FlipFlop,
                    '&' => Module::Conjunction,
                    'b' => Module::Broadcaster,
                    _ => panic!("Unknown module type"),
                };
                (if part_type == Module::Broadcaster { first.to_owned() } else { first.chars().skip(1).collect::<String>().to_owned() },
                 (part_type,
                  rest.split(", ").map(|x| x.to_owned()).collect::<Vec<String>>(),
                  false))
            }
        ).collect::<HashMap<_, (Module, Vec<String>, bool)>>();

    let mut conjunction_receives = parts.iter().filter(|(x, (p, r, s))| *p == Module::Conjunction).map(
        |(x, (p, r, s))| (x.to_owned(), HashMap::new())
    ).collect::<HashMap<_, _>>();
    parts.iter().for_each(|(x, (p, r, s))| {
       for k in r {
           if  let Some(mut conj) = conjunction_receives.get_mut(k) {
               conj.insert(x.to_owned(), false);
           }
       }
    });

    let mut parts = parts;
    let mut low_pulses: u64 = 0;
    let mut high_pulses: u64 = 0;

    for i in 0.. {
        let mut work_set = vec![("broadcaster".to_owned(), false, "".to_owned())];
        // println!("button -low-> broadcaster");
        while !work_set.is_empty() {
            let mut new_work_set = vec![];
            for (work, incoming, from) in work_set {
                if let Some(mut x) = end_conditions.get_mut(&from) {
                    if incoming {
                        if x.0 != 0 {
                            println!("{}: {}", from, i - x.0);
                            x.1 = i;
                        };
                        x.0 = i;
                    }
                }
                if end_conditions.iter().all(|(_, x)| x.1 != 0) {
                    return;
                }
                if !incoming && work == "rx" {
                    println!("{i}");
                    return;
                }
                if incoming {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
                if let Some((module, targets, state)) = parts.get_mut(&work) {
                    match module {
                        Module::Broadcaster => targets.iter().for_each(|x| {
                            // println!("{} -{}-> {}", work, if incoming { "high" } else { "low" }, x);
                            new_work_set.push((x.to_owned(), false, work.to_owned()));
                        }),
                        Module::FlipFlop => {
                            if incoming {
                                continue;
                            }
                            *state = !*state;
                            targets.iter().for_each(|x| {
                                // println!("{} -{}-> {}", work, if *state { "high" } else { "low" }, x);
                                new_work_set.push((x.to_owned(), *state, work.to_owned()));
                            });
                        }
                        Module::Conjunction => {
                            let in_states = conjunction_receives.get_mut(&work).unwrap();
                            *in_states.get_mut(&from).unwrap() = incoming;
                            let con_state = in_states.iter().all(|(_, x)| *x);
                            targets.iter().for_each(|x| {
                                // println!("{} -{}-> {}", work, if !con_state { "high" } else { "low" }, x);
                                new_work_set.push((x.to_owned(), !con_state, work.to_owned()));
                            });
                        }
                    }
                }
            }
            work_set = new_work_set;
        }
        if i == 1000 {
            println!("Low pulses: {}", low_pulses);
            println!("High pulses: {}", high_pulses);
            println!("Total pulses: {}", low_pulses * high_pulses);
        }
    }

}