use std::collections::{BTreeMap, HashMap, HashSet};
use std::collections::btree_map::Entry;
use std::f64::consts::PI;
const ASTEROID: char = '#';
const SPACE: char = '.';

fn count_asteroids(asteroids: Vec<Vec<char>>) -> HashMap<(usize, usize), usize> {
    let mut set_map = HashMap::new();

    for j in 0..asteroids.len() {
        for i in 0..asteroids[j].len() {
            let mut visible_set = HashSet::new();
            if asteroids[j][i] == ASTEROID {
                for l in 0..asteroids.len() {
                    for k in 0..asteroids[j].len() {
                        if i == k && j == l { continue; };
                        if asteroids[l][k] == ASTEROID {
                            let i = i as f64;
                            let j = j as f64;
                            let k = k as f64;
                            let l = l as f64;

                            let angle = (k - i).atan2(l - j);

                            visible_set.insert((angle * 1000.0) as i64);
                        }
                    }
                }
                set_map.insert((i, j), visible_set.len());
            }
        }
    }
    set_map
}

pub fn find_best_location(asteroids: Vec<Vec<char>>) -> ((usize, usize), usize) {
    let mut set_map = count_asteroids(asteroids);
    // let _ = set_map.iter().fold((), |_acc, ((i,j), set)| {
    //     println!("i: {i}, j: {j}, len: {}", set.len());
    // });
    // println!("{:?}", set_map.get(&(11, 13)).unwrap().len());
    // println!("{:?}", set_vec);
    let ((mut i, mut j), mut max_asteroids) = ((0, 0), 0);
    for item in set_map {
        if item.1 > max_asteroids {
            max_asteroids = item.1;
            (i, j) = item.0
        }
    }
    ((i, j), max_asteroids)
}

pub fn destroy_asteroids(asteroids: Vec<Vec<char>>, (i, j): (usize, usize)) {
    let mut destroyable_asteroid_map = asteroids.clone();
    let mut counter = 0;
    loop {
        let mut visible_map = BTreeMap::new();
        for l in 0..destroyable_asteroid_map.len() {
            for k in 0..destroyable_asteroid_map[j].len() {
                if i == k && j == l { continue; };
                if destroyable_asteroid_map[l][k] == ASTEROID {
                    let a = i as f64;
                    let b = j as f64;
                    let c = k as f64;
                    let d = l as f64;

                    let angle = (d - b).atan2(c - a);
                    let mut angle_deg = angle * 1800.0 / PI;
                    if angle_deg < -900.0 {
                        angle_deg += 5400.0;
                    }

                    // insert the closest asteroid
                    match visible_map.entry(angle_deg as i64) {
                        Entry::Occupied(old) => {
                            let (old_k, old_l): &(usize, usize) = old.get();
                            let old_dist = old_k.abs_diff(i) * old_k.abs_diff(i) + old_l.abs_diff(j) * old_l.abs_diff(j);
                            let new_dist = k.abs_diff(i) * k.abs_diff(i) + l.abs_diff(j) * l.abs_diff(j);
                            // let new_dist = (k - i) * (k - i) + (l - j) * (l - j);
                            if new_dist < old_dist {
                                let mutable_old = old.into_mut();
                                mutable_old.0 = k;
                                mutable_old.1 = l;
                            }
                        },
                        Entry::Vacant(v) => {v.insert((k,l));},
                    }
                    // visible_map.insert((angle * 1000.0) as i64, (k, l));
                }
            }
        }
        if visible_map.len() == 0 { break; }

        let mut skip_range = true;

        // start from the angle -90 and above
        // when those asteroids are destroyed do the previous ones in order



        for (angle, (i, j)) in visible_map {
            counter += 1;
            destroyable_asteroid_map[j][i] = SPACE;
            // println!("{counter}# destroyed asteroid: {:?} with angle: {angle}", (i, j));
            if counter == 200 { println!("200th destroyed asteroid: {:?}", (i, j)); }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn new_asteroids() -> Vec<Vec<char>> {
        let str = ".#..##.###...#######
                        ##.############..##.
                        .#.######.########.#
                        .###.#######.####.#.
                        #####.##.#.##.###.##
                        ..#####..#.#########
                        ####################
                        #.####....###.#.#.##
                        ##.#################
                        #####.##.###..####..
                        ..######..##.#######
                        ####.##.####...##..#
                        .#####..#.######.###
                        ##...#.##########...
                        #.##########.#######
                        .####.#.###.###.#.##
                        ....##.##.###..#####
                        .#.#.###########.###
                        #.#.#.#####.####.###
                        ###.##.####.##.#..##";
        str.split('\n').map(|item| {
            if !item.is_empty() {
                item.trim().chars().collect()
            } else {
                "".chars().collect()
            }
        }).collect()
    }

    #[test]
    fn test_parse_asteroids() {
        let asteroids = new_asteroids();
        println!("{:?}", asteroids);
    }

    #[test]
    fn test_find_blocked_asteroids() {
        // save the angles of each asteroid pair for each asteroid into a set,
        // length of the set gives the number of asteroids visible

        let asteroids = new_asteroids();

        let mut set_map = count_asteroids(asteroids.clone());
        // let _ = set_map.iter().fold((), |_acc, ((i,j), set)| {
        //     println!("i: {i}, j: {j}, len: {}", set.len());
        // });
        // println!("{:?}", set_map.get(&(11, 13)).unwrap().len());
        // println!("{:?}", set_vec);
        let ((mut i, mut j), mut max_asteroids) = find_best_location(asteroids.clone());
        println!("{:?}", ((i, j), max_asteroids));
    }

    #[test]
    fn test_destruction() {
        let asteroids = new_asteroids();
        let ((mut i, mut j), mut max_asteroids) = find_best_location(asteroids.clone());
        println!("{:?}", (i, j));
        destroy_asteroids(asteroids.clone(), (i,j));
    }
}