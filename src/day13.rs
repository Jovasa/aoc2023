fn main() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();

    let mut lines = input.lines();

    let mut total = 0;

    loop {
        let mut pattern = vec![];
        loop {
            if let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }
                pattern.push(line.chars().collect::<Vec<char>>());
            } else {
                break;
            }
        }
        if pattern.is_empty() {
            break;
        }
        let width = pattern[0].len();
        let height = pattern.len();

        let mut found = false;
        for x in 0..width - 1 {
            let mut number_incorrect = 0;
            for y in 0..height {
                let mut left = x ;
                let mut right = x + 1;
                while left >= 0 && right < width {
                    if pattern[y][left] != pattern[y][right] {
                        number_incorrect += 1;
                        break;
                    }
                    if left == 0 || right == width - 1 {
                        break;
                    }
                    left -= 1;
                    right += 1;
                }
                if number_incorrect >= 2 {
                    break;
                }
            }
            if number_incorrect == 1 {
                found = true;
                total += x + 1;
                break;
            }
        }

        for y in 0..height - 1 {
            let mut number_incorrect = 0;
            for x in 0..width {
                let mut up = y;
                let mut down = y + 1;
                while up >= 0 && down < height {
                    if pattern[up][x] != pattern[down][x] {
                        number_incorrect += 1;
                        break;
                    }
                    if up == 0 || down == height - 1 {
                        break;
                    }
                    up -= 1;
                    down += 1;
                }
                if number_incorrect >= 2 {
                    break;
                }
            }
            if number_incorrect == 1 {
                found = true;
                total += (y + 1) * 100;
                break;
            }
        }
        if !found {
            panic!("no mirror found");
        }
    }
    println!("{:?}", total);
}