use std::cmp::Ordering;
use std::num::ParseIntError;

#[derive(Clone, Debug)]
pub struct Moon {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
}

impl Moon {
    fn new(str: &str) -> Self {
        let pos = coord_parser(str);
        Self { pos, vel: (0, 0, 0) }
    }

    fn display(&self) {
        println!("{:?}", self);
    }
    fn potential_energy(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }
    fn kinetic_energy(&self) -> i32 {
        self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn step_gravity(&mut self, pair: &mut Moon) {
        let self_pos = self.pos;
        let pair_pos = pair.pos;

        let dx = match self_pos.0.cmp(&pair_pos.0) {
            Ordering::Less => {
                1
            }
            Ordering::Equal => { 0 }
            Ordering::Greater => {
                -1
            }
        };
        let dy = match self_pos.1.cmp(&pair_pos.1) {
            Ordering::Less => {
                1
            }
            Ordering::Equal => { 0 }
            Ordering::Greater => {
                -1
            }
        };
        let dz = match self_pos.2.cmp(&pair_pos.2) {
            Ordering::Less => {
                1
            }
            Ordering::Equal => { 0 }
            Ordering::Greater => {
                -1
            }
        };

        self.vel.0 += dx;
        self.vel.1 += dy;
        self.vel.2 += dz;
        pair.vel.0 -= dx;
        pair.vel.1 -= dy;
        pair.vel.2 -= dz;
    }
}

fn coord_parser(str: &str) -> (i32, i32, i32) {
    // "<x=" + num1 + ", y=" + num2 + ", z=" + num3 + ">";
    let iterator = str.split_terminator(&['=', ',', '>', '<', '\n'][..]);
    let mut vector = vec![];
    for n in iterator {
        match n.parse::<i32>() {
            Ok(num) => { vector.push(num) }
            Err(_) => {}
        }
    }
    (vector[0], vector[1], vector[2])
}

fn parse_moons(str: Vec<&str>) -> Vec<Moon> {
    let mut moons = vec![];
    for s in str {
        moons.push(Moon::new(s));
    }
    moons
}

fn calculate_velocities(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() - 1 {
        let (left, right) = moons.split_at_mut(i + 1);
        for pair in right {
            // println!("moon: {:?}, pair: {:?}", left[i], pair);
            left[i].step_gravity(pair)
        }
    }
}

fn calculate_new_locations(moons: &mut Vec<Moon>) {
    for moon in moons {
        moon.pos.0 += moon.vel.0;
        moon.pos.1 += moon.vel.1;
        moon.pos.2 += moon.vel.2;
    }
}
fn full_steps_n(moons: &mut Vec<Moon>, n: usize) {
    for i in 0..n {
        calculate_velocities(moons);
        calculate_new_locations(moons);
    }
}

pub fn calculate_total_energy(str: Vec<&str>, n: usize) -> i32 {
    let mut moons = parse_moons(str);
    full_steps_n(&mut moons, n);

    moons.iter().fold(0, |acc, moon| {
        acc + moon.total_energy()
    })
}
pub fn full_steps_energy_period(moons: &mut Vec<Moon>, n: usize) -> usize {
    let (initial_pos, initial_vel, initial_energy) = moons.iter().fold((vec![], vec![], 0), |mut acc, moon| {
        acc.0.push(moon.pos);
        acc.1.push(moon.vel);
        acc.2 += moon.total_energy();
        acc
    });
    for i in 0..n {
        calculate_velocities(moons);
        calculate_new_locations(moons);

        if calculate_current_total_energy(moons.clone()) == initial_energy {
            let (cur_pos, cur_vel, cur_energy) = moons.iter().fold((vec![], vec![], 0), |mut acc, moon| {
                acc.0.push(moon.pos);
                acc.1.push(moon.vel);
                acc.2 += moon.total_energy();
                acc
            });
            if initial_pos == cur_pos && initial_vel == cur_vel {
                println!("in{:?}", initial_pos);
                println!("cur{:?}", cur_pos);
                println!("inv{:?}", initial_vel);
                println!("curv{:?}", cur_vel);
                return i;
            }
        }
    }
    0
}

pub fn calculate_current_total_energy(moons: Vec<Moon>) -> i32 {
    moons.iter().fold(0, |acc, moon| {
        acc + moon.total_energy()
    })
}

//naive approach
pub fn calculate_repeating_index(str: Vec<&str>, n: usize) -> usize {
    let mut moons = parse_moons(str);
    full_steps_energy_period(&mut moons, n)
}


fn full_steps_periods(moons: &mut Vec<Moon>, n: usize) {
    let mut x_locs = vec![vec![]; 4];
    let mut y_locs = vec![vec![]; 4];
    let mut z_locs = vec![vec![]; 4];

    let mut x_vels = vec![vec![]; 4];
    let mut y_vels = vec![vec![]; 4];
    let mut z_vels = vec![vec![]; 4];
    for i in 0..n {
        calculate_velocities(moons);
        calculate_new_locations(moons);
        x_locs[0].push(moons[0].pos.0);
        x_locs[1].push(moons[1].pos.0);
        x_locs[2].push(moons[2].pos.0);
        x_locs[3].push(moons[3].pos.0);

        y_locs[0].push(moons[0].pos.1);
        y_locs[1].push(moons[1].pos.1);
        y_locs[2].push(moons[2].pos.1);
        y_locs[3].push(moons[3].pos.1);

        z_locs[0].push(moons[0].pos.2);
        z_locs[1].push(moons[1].pos.2);
        z_locs[2].push(moons[2].pos.2);
        z_locs[3].push(moons[3].pos.2);

        x_vels[0].push(moons[0].vel.0);
        x_vels[1].push(moons[1].vel.0);
        x_vels[2].push(moons[2].vel.0);
        x_vels[3].push(moons[3].vel.0);

        y_vels[0].push(moons[0].vel.1);
        y_vels[1].push(moons[1].vel.1);
        y_vels[2].push(moons[2].vel.1);
        y_vels[3].push(moons[3].vel.1);

        z_vels[0].push(moons[0].vel.2);
        z_vels[1].push(moons[1].vel.2);
        z_vels[2].push(moons[2].vel.2);
        z_vels[3].push(moons[3].vel.2);
    }


    println!("Position repetitions");
    let x0pat = find_pattern(x_locs[0].clone());
    let x1pat = find_pattern(x_locs[1].clone());
    let x2pat = find_pattern(x_locs[2].clone());
    let x3pat = find_pattern(x_locs[3].clone());
    println!("len: {}, {:?}", x0pat.len(), x0pat);
    println!("len: {}, {:?}", x1pat.len(), x1pat);
    println!("len: {}, {:?}", x2pat.len(), x2pat);
    println!("len: {}, {:?}", x3pat.len(), x3pat);

    let y0pat = find_pattern(y_locs[0].clone());
    let y1pat = find_pattern(y_locs[1].clone());
    let y2pat = find_pattern(y_locs[2].clone());
    let y3pat = find_pattern(y_locs[3].clone());
    println!("len: {}, {:?}", y0pat.len(), y0pat);
    println!("len: {}, {:?}", y1pat.len(), y1pat);
    println!("len: {}, {:?}", y2pat.len(), y2pat);
    println!("len: {}, {:?}", y3pat.len(), y3pat);

    let z0pat = find_pattern(z_locs[0].clone());
    let z1pat = find_pattern(z_locs[1].clone());
    let z2pat = find_pattern(z_locs[2].clone());
    let z3pat = find_pattern(z_locs[3].clone());
    println!("len: {}, {:?}", z0pat.len(), z0pat);
    println!("len: {}, {:?}", z1pat.len(), z1pat);
    println!("len: {}, {:?}", z2pat.len(), z2pat);
    println!("len: {}, {:?}", z3pat.len(), z3pat);

    println!("Velocity repetitions");

    let x0pat = find_pattern(x_locs[0].clone());
    let x1pat = find_pattern(x_locs[1].clone());
    let x2pat = find_pattern(x_locs[2].clone());
    let x3pat = find_pattern(x_locs[3].clone());
    println!("len: {}, {:?}", x0pat.len(), x0pat);
    println!("len: {}, {:?}", x1pat.len(), x1pat);
    println!("len: {}, {:?}", x2pat.len(), x2pat);
    println!("len: {}, {:?}", x3pat.len(), x3pat);

    let y0pat = find_pattern(y_locs[0].clone());
    let y1pat = find_pattern(y_locs[1].clone());
    let y2pat = find_pattern(y_locs[2].clone());
    let y3pat = find_pattern(y_locs[3].clone());
    println!("len: {}, {:?}", y0pat.len(), y0pat);
    println!("len: {}, {:?}", y1pat.len(), y1pat);
    println!("len: {}, {:?}", y2pat.len(), y2pat);
    println!("len: {}, {:?}", y3pat.len(), y3pat);

    let z0pat = find_pattern(z_locs[0].clone());
    let z1pat = find_pattern(z_locs[1].clone());
    let z2pat = find_pattern(z_locs[2].clone());
    let z3pat = find_pattern(z_locs[3].clone());
    println!("len: {}, {:?}", z0pat.len(), z0pat);
    println!("len: {}, {:?}", z1pat.len(), z1pat);
    println!("len: {}, {:?}", z2pat.len(), z2pat);
    println!("len: {}, {:?}", z3pat.len(), z3pat);

    let lengths_vec = vec![x0pat.len(), y0pat.len(), z0pat.len(),
                           x1pat.len(), y1pat.len(), z1pat.len(),
                           x2pat.len(), y2pat.len(), z2pat.len(),
                           x3pat.len(), y3pat.len(), z3pat.len(), ];

    let lcm = lengths_vec.iter().fold(1, |mut acc, item| {
        acc = lcm(acc, *item as u128);
        acc
    });
    println!("{:?}", lengths_vec);
    println!("{}", lcm);


    // println!("{:?}", x_locs.clone());

}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd(a, b)
}

pub fn calculate_repetition_period(str: Vec<&str>, n: usize) -> u128{
    let mut moons = parse_moons(str);
    let mut x_locs = vec![vec![]; 4];
    let mut y_locs = vec![vec![]; 4];
    let mut z_locs = vec![vec![]; 4];

    let mut x_vels = vec![vec![]; 4];
    let mut y_vels = vec![vec![]; 4];
    let mut z_vels = vec![vec![]; 4];
    for i in 0..n {
        calculate_velocities(&mut moons);
        calculate_new_locations(&mut moons);
        x_locs[0].push(moons[0].pos.0);
        x_locs[1].push(moons[1].pos.0);
        x_locs[2].push(moons[2].pos.0);
        x_locs[3].push(moons[3].pos.0);

        y_locs[0].push(moons[0].pos.1);
        y_locs[1].push(moons[1].pos.1);
        y_locs[2].push(moons[2].pos.1);
        y_locs[3].push(moons[3].pos.1);

        z_locs[0].push(moons[0].pos.2);
        z_locs[1].push(moons[1].pos.2);
        z_locs[2].push(moons[2].pos.2);
        z_locs[3].push(moons[3].pos.2);

        x_vels[0].push(moons[0].vel.0);
        x_vels[1].push(moons[1].vel.0);
        x_vels[2].push(moons[2].vel.0);
        x_vels[3].push(moons[3].vel.0);

        y_vels[0].push(moons[0].vel.1);
        y_vels[1].push(moons[1].vel.1);
        y_vels[2].push(moons[2].vel.1);
        y_vels[3].push(moons[3].vel.1);

        z_vels[0].push(moons[0].vel.2);
        z_vels[1].push(moons[1].vel.2);
        z_vels[2].push(moons[2].vel.2);
        z_vels[3].push(moons[3].vel.2);
    }

    let x0pat = find_pattern(x_locs[0].clone());
    let x1pat = find_pattern(x_locs[1].clone());
    let x2pat = find_pattern(x_locs[2].clone());
    let x3pat = find_pattern(x_locs[3].clone());

    let y0pat = find_pattern(y_locs[0].clone());
    let y1pat = find_pattern(y_locs[1].clone());
    let y2pat = find_pattern(y_locs[2].clone());
    let y3pat = find_pattern(y_locs[3].clone());

    let z0pat = find_pattern(z_locs[0].clone());
    let z1pat = find_pattern(z_locs[1].clone());
    let z2pat = find_pattern(z_locs[2].clone());
    let z3pat = find_pattern(z_locs[3].clone());

    let x0pat = find_pattern(x_locs[0].clone());
    let x1pat = find_pattern(x_locs[1].clone());
    let x2pat = find_pattern(x_locs[2].clone());
    let x3pat = find_pattern(x_locs[3].clone());

    let y0pat = find_pattern(y_locs[0].clone());
    let y1pat = find_pattern(y_locs[1].clone());
    let y2pat = find_pattern(y_locs[2].clone());
    let y3pat = find_pattern(y_locs[3].clone());

    let z0pat = find_pattern(z_locs[0].clone());
    let z1pat = find_pattern(z_locs[1].clone());
    let z2pat = find_pattern(z_locs[2].clone());
    let z3pat = find_pattern(z_locs[3].clone());

    let lengths_vec = vec![x0pat.len(), y0pat.len(), z0pat.len(),
                           x1pat.len(), y1pat.len(), z1pat.len(),
                           x2pat.len(), y2pat.len(), z2pat.len(),
                           x3pat.len(), y3pat.len(), z3pat.len()];

    let lcm = lengths_vec.iter().fold(1, |mut acc, item| {
        acc = lcm(acc, *item as u128);
        acc
    });
    lcm

}

fn find_pattern(vec: Vec<i32>) -> Vec<i32> {
    for pattern_length in 1..=vec.len() / 2 {
        let mut pattern = vec[0..pattern_length].to_vec();
        let mut is_pattern = true;

        for i in pattern_length..vec.len() {
            if vec[i] != pattern[i % pattern_length] {
                is_pattern = false;
                break;
            }
        }

        if is_pattern {
            return pattern;
        }
    }

    vec // If no pattern is found, return the entire vector as the "pattern"
}
//
// fn find_period(vec: Vec<i32>) -> usize {
//     let mut repeat_index;
//     let mut repeat_max = 0;
//     for i in 1..vec.len() - 1 {
//         repeat_index = 0;
//         if vec[i] == vec[repeat_index] && 2 * i <= vec.len() {
//             // println!("matching indices: {i}!");
//             for j in i..2 * i {
//                 // println!("j: {}, repeat_index: {}", j, repeat_index);
//                 if vec[j] == vec[repeat_index] { repeat_index += 1 } else { break; }
//                 if j == 2*i-1 { repeat_max = repeat_index }
//             }
//             // 112211223112211223
//             // if repeat_max < repeat_index { repeat_max = repeat_index }
//             // if repeat_index == 2*i - 1 { return repeat_max }
//         }
//     }
//     repeat_max
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsing() {
        let str = "<x=17, y=-9, z=4>";
        println!("{:?}", coord_parser(str));
    }
    #[test]
    fn test_moon() {
        let moon = Moon::new("<x=17, y=-9, z=4>");
        println!("{:?}", moon);
    }
    #[test]
    fn test_moons() {
        let moon = parse_moons(vec!["<x=17, y=-9, z=4>", "<x=11, y=2, z=-14>", "<x=-1, y=-2, z=3>"]);
        println!("{:?}", moon);
    }
    #[test]
    fn test_steps() {
        let mut moons = parse_moons(vec!["<x=-1, y=0, z=2>", "<x=2, y=-10, z=-7>", "<x=4, y=-8, z=8>", "<x=3, y=5, z=-1>"]);
        calculate_velocities(&mut moons);
        for moon in moons.clone() {
            moon.display();
        }
        calculate_new_locations(&mut moons);
        println!("New locations!");
        for moon in moons {
            moon.display();
        }
    }

    #[test]
    fn test_full_steps() {
        let mut moons = parse_moons(vec!["<x=-1, y=0, z=2>", "<x=2, y=-10, z=-7>", "<x=4, y=-8, z=8>", "<x=3, y=5, z=-1>"]);
        full_steps_n(&mut moons, 2772);
        for moon in moons {
            moon.display();
        }
    }

    #[test]
    fn test_energy() {
        let mut moons = parse_moons(vec!["<x=-8, y=-10, z=0>",
                                         "<x=2, y=-10, z=-7>",
                                         "<x=4, y=-8, z=8>",
                                         "<x=3, y=5, z=-1>"]);
        full_steps_n(&mut moons, 100);
        for moon in moons {
            moon.display();
            println!("energy: {}", moon.total_energy());
        }
    }

    #[test]
    fn test_energy_periods() {
        let mut moons = parse_moons(vec!["<x=-1, y=0, z=2>",
                                         "<x=2, y=-10, z=-7>",
                                         "<x=4, y=-8, z=8>",
                                         "<x=3, y=5, z=-1>"]);
        println!("{}", full_steps_energy_period(&mut moons, 10000));
    }


    #[test]
    fn test_periods() {
        let mut moons = parse_moons(vec!["<x=-1, y=0, z=2>",
                                         "<x=2, y=-10, z=-7>",
                                         "<x=4, y=-8, z=8>",
                                         "<x=3, y=5, z=-1>"]);

        full_steps_periods(&mut moons, 1000);
    }

    #[test]
    fn test_patterns() {
        // let mut moons = parse_moons(vec!["<x=-1, y=0, z=2>",
        //                                  "<x=5, y=5, z=10>",
        //                                  "<x=2, y=-7, z=3>",
        //                                  "<x=9, y=-8, z=-3>"]);
        //
        // full_steps_periods(&mut moons, 1000);
        let vec = vec![1, 1, 2, 2, 1, 1, 2, 2, 3, 1, 1, 2, 2, 1, 1, 2, 2, 3, 1, 1, 2, 2, 1, 1, 2, 2, 3, 1];
        // let vec = vec![1, 1, 2, 2, 1,1 ,2,2];

        let pat = find_pattern(vec.clone());
        println!("{:?}", pat);
    }
}