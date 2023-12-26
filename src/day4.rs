use std::collections::HashSet;
use std::fs;

fn main() {
    let input_file = "data/day4.txt";
    let card_list = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Error reading the input file");
            return;
        }
    };

    // card are of form:
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    task1(&card_list);
    task2(&card_list);
}

fn task1(card_list: &str) {
    let mut total_score = 0;

    for line in card_list.lines() {
        let winnings = get_winnings(line);
        if winnings >= 1 {
            total_score += 1 << (winnings - 1);
        }
    }
    println!("total score: {}", total_score);
}

fn task2(card_list: &str) {
    let lines = card_list.lines().collect::<Vec<&str>>();

    let mut tickets = vec![1; lines.len()];

    for i in 0..lines.len() {
        let line = lines[i];
        let winnings = get_winnings(line);

        let current_ticket_count = tickets[i];
        if winnings >= 1 {
            for j in 1..=winnings {
                tickets[i + j] += current_ticket_count;
            }
        }
    }

    // print total ticket count
    println!("total ticket count: {}", tickets.iter().sum::<usize>());
}

fn get_winnings(line: &str) -> usize {
    let cards = line.split(":").skip(1).next().unwrap().to_owned();
    let mut separated_cards = cards.split("|");
    let winning_cards = separated_cards
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s|
            s.trim().parse().unwrap())
        .collect::<HashSet<u32>>();
    let my_numbers = separated_cards
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| { s.trim().parse().unwrap() })
        .collect::<HashSet<u32>>();

    let winnings = winning_cards.intersection(&my_numbers).count();
    winnings
}