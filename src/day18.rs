use std::ops::{Add, Div};

fn manhattan_distance((x1, y1): &(i64, i64), (x2, y2): &(i64, i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn shoelace_area(points: &[(i64, i64)]) -> i64 {
    let len = points.len();

    let (area, perimeter) =
        points
            .iter()
            .enumerate()
            .fold((0, 0), |(sum, perimeter), (i, p1)| {
                let l = (i + 1) % len;
                let p2 = points[l];

                let new_perimeter = perimeter + manhattan_distance(&p1, &p2);
                let new_area = sum + (p1.1 * p2.0) - (p1.0 * p2.1);

                (new_area, new_perimeter)
            });

    area.abs().add(perimeter).div(2).add(1)
}

fn main() {
    let input = std::fs::read_to_string("data/day18.txt").unwrap();

    part1(input.clone());
    part2(input);
}

fn part2(input: String) {
    let mut points = vec![(0, 0)];

    let mut last = (0i64, 0i64);
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let value = parts[2].chars().skip(2).take(5).collect::<String>();
        let value = i64::from_str_radix(&value, 16).unwrap();
        let next = match parts[2].chars().skip(7).next().unwrap() {
            '2' => (last.0 - value, last.1),
            '0' => (last.0 + value, last.1),
            '3' => (last.0, last.1 - value),
            '1' => (last.0, last.1 + value),
            _ => panic!()
        };
        points.push(next);
        last = next;
    }
    let area = shoelace_area(&points);
    println!("{}", area);

}

fn part1(input: String) {
    let mut points = vec![(0, 0)];

    let mut last = (0i64, 0i64);
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let next = match parts[0].chars().next().unwrap() {
            'L' => (last.0 - parts[1].parse::<i64>().unwrap(), last.1),
            'R' => (last.0 + parts[1].parse::<i64>().unwrap(), last.1),
            'U' => (last.0, last.1 - parts[1].parse::<i64>().unwrap()),
            'D' => (last.0, last.1 + parts[1].parse::<i64>().unwrap()),
            _ => panic!()
        };
        points.push(next);
        last = next;
    }
    let area = shoelace_area(&points);
    println!("{}", area);
}