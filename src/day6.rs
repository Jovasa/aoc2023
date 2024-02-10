fn main() {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    let mut lines = input.lines();

    let times = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut total = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        println!("{} {}", time, distance);
        let mut times_beaten = 0;
        for i in 0..*time {
            let time_left = time - i;
            let total_distance = i * time_left;
            if total_distance >= *distance {
                times_beaten += 1;
            }
        }
        total *= times_beaten;
    }

    println!("total: {}", total);

    let a = -1i64;
    let b = 55_99_97_93i64;
    let c = -401_1485_2274_1405i64;

    let determinant = b*b - 4*a*c;
    let det_root = (determinant as f64).sqrt() as i64;
    let x1 = (-b + det_root) / (2*a);
    let x2 = (-b - det_root) / (2*a);
    println!("x1: {}, x2: {}", x1, x2);
    println!("{}", x2 - x1); // Plus one because of some reason
}