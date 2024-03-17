use std::collections::HashSet;

#[derive(Debug)]
struct Hail {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

fn main() {
    let input = std::fs::read_to_string("data/day24.txt").unwrap();

    let hails = input.lines().map(|l| {
        let mut t = l.split(" @ ");
        let positions = t.next().unwrap().split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let velocities = t.next().unwrap().split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Hail {
            x: positions[0],
            y: positions[1],
            z: positions[2],
            vx: velocities[0],
            vy: velocities[1],
            vz: velocities[2],
        }
    }).collect::<Vec<_>>();

    let mut total = 0;

    let mut x_velocities = HashSet::new();
    let mut y_velocities = HashSet::new();
    let mut z_velocities = HashSet::new();

    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            let hail1 = &hails[i];
            let hail2 = &hails[j];

            let hail1_point_1 = (hail1.x as f64, hail1.y as f64);
            let hail1_point_2 = ((hail1.x + hail1.vx) as f64, (hail1.y + hail1.vy) as f64);

            let hail2_point_1 = (hail2.x as f64, hail2.y as f64);
            let hail2_point_2 = ((hail2.x + hail2.vx) as f64, (hail2.y + hail2.vy) as f64);

            let a1 = (hail1_point_2.1 - hail1_point_1.1) / (hail1_point_2.0 - hail1_point_1.0);
            let b1 = hail1_point_1.1 - a1 * hail1_point_1.0;

            let a2 = (hail2_point_2.1 - hail2_point_1.1) / (hail2_point_2.0 - hail2_point_1.0);
            let b2 = hail2_point_1.1 - a2 * hail2_point_1.0;

            let intersect_x = (b2 - b1) / (a1 - a2);
            let intersect_y = a1 * intersect_x + b1;
            let lower_limit =  200000000000000f64;
            let upper_limit =  400000000000000f64;
            if lower_limit < intersect_x && intersect_x < upper_limit
                && lower_limit < intersect_y && intersect_y < upper_limit {
                if ((hail1.vx > 0 && intersect_x > hail1.x as f64)
                    || (hail1.vx < 0 && intersect_x < hail1.x as f64))
                    && ((hail2.vx > 0 && intersect_x > hail2.x as f64)
                    || (hail2.vx < 0 && intersect_x < hail2.x as f64)) {
                    total += 1;
                    // println!("{:?} {:?} {} {}", hail1, hail2, intersect_x, intersect_y)
                }
            }

            if hail1.vx == hail2.vx {
                let mut possible_velocities = HashSet::new();
                for v in -1000..1000 {
                    if v - hail1.vx == 0 {
                        continue;
                    }
                    if (hail1.x - hail2.x) % (v - hail1.vx) == 0 {
                        possible_velocities.insert(v);
                    }
                }
                if x_velocities.is_empty() {
                    x_velocities = possible_velocities;
                } else {
                    x_velocities = x_velocities.intersection(&possible_velocities).cloned().collect();
                }
            }
            if hail1.vy == hail2.vy {
                let mut possible_velocities = HashSet::new();
                for v in -1000..1000 {
                    if v -hail1.vy == 0 {
                        continue;
                    }
                    if (hail1.y - hail2.y) % (v - hail1.vy) == 0 {
                        possible_velocities.insert(v);
                    }
                }
                if y_velocities.is_empty() {
                    y_velocities = possible_velocities;
                } else {
                    y_velocities = y_velocities.intersection(&possible_velocities).cloned().collect();
                }
            }
            if hail1.vz == hail2.vz {
                let mut possible_velocities = HashSet::new();
                for v in -1000..1000 {

                    if v -hail1.vz == 0 {
                        continue;
                    }
                    if (hail1.z - hail2.z) % (v - hail1.vz) == 0 {
                        possible_velocities.insert(v);
                    }
                }
                if z_velocities.is_empty() {
                    z_velocities = possible_velocities;
                } else {
                    z_velocities = z_velocities.intersection(&possible_velocities).cloned().collect();
                }
            }
        }
    }
    println!("{}", total);
    println!("{:?} {:?} {:?}", x_velocities, y_velocities, z_velocities);

    let x_vel = x_velocities.iter().next().unwrap();
    let y_vel = y_velocities.iter().next().unwrap();
    let z_vel = z_velocities.iter().next().unwrap();

    let hail1 = &hails[0];
    let hail2 = &hails[1];

    let ma = (hail1.vy - y_vel) as f64 / (hail1.vx - x_vel) as f64;
    let mb = (hail2.vy - y_vel) as f64 / (hail2.vx - x_vel) as f64;

    let ca = hail1.y as f64 - ma * hail1.x as f64;
    let cb = hail2.y as f64 - mb * hail2.x as f64;

    let intersect_x = (cb - ca) / (ma - mb);
    let intersect_y = ma * intersect_x + ca;
    let time = (intersect_x - hail1.x as f64) / (hail1.vx - x_vel) as f64;
    let intersect_z = (hail1.z as f64) + time * (hail1.vz - z_vel) as f64;

    println!("{:?} {:?} {:?}", intersect_x, intersect_y, intersect_z);
    println!("{:?}", intersect_y + intersect_x + intersect_z);
}