fn main() {
    // read input file
    let input = std::fs::read_to_string("data/day2.txt").unwrap();
    // split input into lines
    let lines = input.lines();

    // Lines are of the form:
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    let mut sum = 0;

    let mut powers = 0;

    // iterate over lines
    for line in lines {
        let temp = line.split(": ").collect::<Vec<&str>>();
        let game_id = temp[0];
        let rest = temp[1];


        // split line into games
        let games = rest.split("; ");

        let mut valid = true;

        let mut minimum_required = [0; 3];

        // iterate over games
        for game in games {
            // Count all different colored tokens in each game
            let mut totals = [0; 3];
            // split game into piles
            let piles = game.split(", ");
            // iterate over piles
            for pile in piles {
                // split pile into tokens
                let tokens = pile.split(" ");
                // iterate over tokens
                for token in tokens {
                    // if token is a number, add it to the sum
                    if let Ok(number) = token.parse::<u32>() {
                        if pile.ends_with("blue") {
                            totals[0] += number;
                        } else if pile.ends_with("red") {
                            totals[1] += number;
                        } else if pile.ends_with("green") {
                            totals[2] += number;
                        }
                    }
                }
            }
            if !(totals[0] <= 14 && totals[1] <= 12 && totals[2] <= 13) {
                valid = false;
            }

            if totals[0] > minimum_required[0] {
                minimum_required[0] = totals[0];
            }
            if totals[1] > minimum_required[1] {
                minimum_required[1] = totals[1];
            }
            if totals[2] > minimum_required[2] {
                minimum_required[2] = totals[2];
            }
        }

        if valid {
            sum += game_id.split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        }
        powers += minimum_required[0] * minimum_required[1] * minimum_required[2];
    }
    println!("sum: {}", sum);
    println!("powers: {}", powers);
}