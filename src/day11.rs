use std::collections::{ HashSet};

fn main() {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();

    let stars = input.lines().enumerate().map(|(y, line)| {
        line.bytes().enumerate().filter_map(move |(x, c)| {
            if c == b'#' {
                Some((x as i32 , y as i32 ))
            } else {
                None
            }
        })
    }).flatten().collect::<Vec<_>>();
    println!("{:?}", stars.len());

    let max_x = stars.iter().map(|(x, _)| x).max().unwrap();
    let max_y = stars.iter().map(|(_, y)| y).max().unwrap();
    let rows_with_stars = stars.iter().map(|(_, y)| *y).collect::<HashSet<_>>();
    let cols_with_stars = stars.iter().map(|(x, _)| *x).collect::<HashSet<_>>();

    let rows_without_stars = (0..=*max_y).filter(|y| !rows_with_stars.contains(y)).collect::<HashSet<_>>();
    let cols_without_stars = (0..=*max_x).filter(|x| !cols_with_stars.contains(x)).collect::<HashSet<_>>();

    let mut total_distances = 0;
    for i in 0..stars.len() {
        let (x, y) = stars[i];
        for j in i + 1..stars.len() {
            let (x2, y2) = stars[j];
            let cols_between_without_stars = (x.min(x2) + 1..x2.max(x)).filter(|x| cols_without_stars.contains(x)).count();
            let rows_between_without_stars = (y.min(y2) + 1..y2.max(y)).filter(|y| rows_without_stars.contains(y)).count();
            let distance = ((x2 - x).abs() as usize  + cols_between_without_stars *999_999) + ((y2 - y).abs() as usize + rows_between_without_stars *999_999);
            total_distances += distance as usize;
            // println!("{} {}: {} {}", i + 1, j + 1, distance, total_distances)
        }
    }
    println!("{:?}", rows_without_stars);
    println!("{:?}", cols_without_stars);
    println!("{:?}", total_distances);
}