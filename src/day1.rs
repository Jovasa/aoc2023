fn find_first_digit(input: &str) -> Option<u32> {
    for c in input.chars() {
        if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap());
        }
    }
    None
}

fn find_first_digit_of_named_number(input: &str, is_reverse: bool) -> Option<u32> {
    let number_names = if !is_reverse {
        vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    } else {
        vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"]
    };

    for i in 0..input.len() {
        if input.as_bytes()[i].is_ascii_digit() {
            return Some((input.as_bytes()[i] - ('0' as u8)) as u32 );
        }
        for j in 0..number_names.len() {
            if input[i..].starts_with(number_names[j]) {
                return Some((j + 1) as u32);
            }
        }
    }
    None
}

fn main() {
    // read input file
    let input = std::fs::read_to_string("data/day1.txt").unwrap();
    // split input into lines
    let lines = input.lines();

    let mut sum = 0;
    // iterate over lines
    for line in lines {
        // find first digit
        let first_digit = find_first_digit_of_named_number(line, false).unwrap() * 10;
        let last_digit = find_first_digit_of_named_number(&line.chars().rev().collect::<String>(), true).unwrap();
        sum += first_digit + last_digit;
    }
    println!("sum: {}", sum);
}