use std::cmp::Ordering;

fn card_to_value(card: char, joker_rule: bool) -> u32 {
    if joker_rule {
        return card_to_value_joker(card);
    }
    match card {
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => card.to_digit(10).unwrap() - 2,
    }
}

fn card_to_value_joker(card: char) -> u32 {
    match card {
        'T' => 9,
        'J' => 0,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => card.to_digit(10).unwrap() - 1,
    }
}

fn is_fullhouse(counts: &[u32; 13]) -> bool {
    let mut has_3 = false;
    let mut has_2 = false;
    for count in counts.iter() {
        if *count == 3 {
            has_3 = true;
        } else if *count == 2 {
            has_2 = true;
        }
    }
    has_3 && has_2
}

fn handle_joker(counts: &mut [u32; 13]) {
    let num_of_jokers = counts[0];
    if num_of_jokers == 0 {
        return;
    }
    let mut index_of_highest_card = 0;
    let mut highest_card = 0;
    for (i, count) in counts.iter().enumerate().skip(1) {
        if *count > highest_card {
            highest_card = *count;
            index_of_highest_card = i;
        }
    }
    counts[0] = 0;
    counts[index_of_highest_card] += num_of_jokers;
}

fn compare_hands(first: &str, second: &str, joker_rule: bool) -> Ordering {
    // count the number of occurences of each card
    let mut first_counts = [0; 13];
    let mut second_counts = [0; 13];
    for card in first.chars().take(5) {
        let value =  card_to_value(card, joker_rule);
        first_counts[value as usize] += 1;
    }
    for card in second.chars().take(5) {
        let value = card_to_value(card, joker_rule);
        second_counts[value as usize] += 1;
    }
    if joker_rule {
        handle_joker(&mut first_counts);
        handle_joker(&mut second_counts);
    }
    let maximum_number_of_cards_in_hand1 = *first_counts.iter().max().unwrap();
    let maximum_number_of_cards_in_hand2 = *second_counts.iter().max().unwrap();
    if maximum_number_of_cards_in_hand1 > maximum_number_of_cards_in_hand2 {
        return Ordering::Greater;
    } else if maximum_number_of_cards_in_hand1 < maximum_number_of_cards_in_hand2 {
        return Ordering::Less;
    }
    if is_fullhouse(&first_counts) {
        if !is_fullhouse(&second_counts) {
            return Ordering::Greater;
        }
    } else if is_fullhouse(&second_counts) {
        return Ordering::Less;
    }

    if maximum_number_of_cards_in_hand1 == 2  {
        let count_of_2_in_hand1 = first_counts.iter().filter(|x| **x == 2).count();
        let count_of_2_in_hand2 = second_counts.iter().filter(|x| **x == 2).count();
        if count_of_2_in_hand1 > count_of_2_in_hand2 {
            return Ordering::Greater;
        } else if count_of_2_in_hand1 < count_of_2_in_hand2 {
            return Ordering::Less;
        }
    }
    for (f, s) in first.chars().zip(second.chars()).take(5) {
        let f = card_to_value(f, joker_rule);
        let s = card_to_value(s, joker_rule);
        if f > s {
            return Ordering::Greater;
        } else if f < s {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn main() {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();
    let mut lines = input.lines().collect::<Vec<&str>>();
    lines.sort_by(|a, b| compare_hands(a, b, false));
    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        let bet = line.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total += bet * (i + 1);
    }
    //250058342
    println!("total: {}", total);

    lines.sort_by(|a, b| compare_hands(a, b, true));
    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        let bet = line.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total += bet * (i + 1);
    }
    //250506580
    println!("total: {}", total);
}