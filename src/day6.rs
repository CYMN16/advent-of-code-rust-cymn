use std::collections::BTreeSet;
use std::collections::HashMap;
// struct OrbitTree {
//     left: Option<Box<OrbitTree>>,
//     right: Option<Box<OrbitTree>>,
// }
//
// impl OrbitTree {
//     fn new(name: &str) -> Self {
//         Self { left: None, right: None }
//     }
//
//     fn insert(&mut self, child: &mut OrbitTree) {
//
//     }
// }
#[derive(Clone, PartialEq, Debug)]
pub struct OrbitPair {
    center_of_mass: String,
    orbiter: String,
}

impl OrbitPair {
    fn new(center_of_mass: &str, orbiter: &str) -> Self {
        Self { center_of_mass: center_of_mass.to_string(), orbiter: orbiter.to_string() }
    }
}

pub fn parse_pairs(vec: &Vec<&str>) -> Vec<OrbitPair> {
    let mut pair_vec: Vec<OrbitPair> = vec![];
    for item in vec {
        let val: Vec<&str> = item.split(')').map(|item| {
            if !item.is_empty() {
                item
            } else {
                ""
            }
        }).collect();

        // println!("{:?}", val);
        // pair_vec.push((val[0].to_string(), val[1].to_string()));
        pair_vec.push(OrbitPair { center_of_mass: val[0].to_string(), orbiter: val[1].to_string() })
    }
    pair_vec
}

pub fn insert_orbits_to_hashmap(vec: Vec<OrbitPair>) -> HashMap<String, Vec<String>> {
    let mut orbit_map: HashMap<String, Vec<String>> = HashMap::new();
    for pair in vec {
        orbit_map.entry(pair.center_of_mass.to_string()).or_insert(vec![]).push(pair.orbiter.to_string());
    }
    orbit_map
}

pub fn find_leftmost_com(orbit_map: &HashMap<String, Vec<String>>) -> Result<String, String> {
    let mut left_set = BTreeSet::new();
    let mut right_set = BTreeSet::new();

    for (key, val) in orbit_map {
        left_set.insert(key);
        for child in val {
            right_set.insert(child);
        }
    }

    let differences: Vec<&&String> = left_set.difference(&right_set).collect();

    match differences.len() {
        1 => { Ok(differences[0].to_string()) }
        _ => { Err("unmatched number of differences".to_string()) }
    }
}

pub fn count_orbits(orbit_map: HashMap<String, Vec<String>>) -> u32 {
    let mut seen_set: BTreeSet<String> = BTreeSet::new();
    let mut depth_map: HashMap<String, u32> = HashMap::new();
    let com = find_leftmost_com(&orbit_map).unwrap();
    let mut stack: Vec<String> = vec![com];
    let mut depth = 0;
    let separator = "---===---".to_string();
    loop {
        let current_str = stack.last().clone();

        let current_str = match current_str {
            None => { break }
            Some(val) => {
                // println!("{val}");
                if *val == separator {
                    stack.pop();
                    depth -= 1;
                    continue;
                } else {
                    val
                }
            }
        };
        // println!("{}", current_com);
        let mut orbits = orbit_map.get(current_str);
        match seen_set.insert(current_str.to_string()) {
            true => {}
            false => {
                // println!("{:?}", stack.pop());
                depth_map.insert(stack.pop().unwrap(), depth);
                // counter += 1;
            }
        };

        let orbits = match orbits {
            None => {
                continue;
            }
            Some(val) => {
                // push an indicator because a new depth is achieved and increase the depth
                stack.push(separator.clone());
                depth += 1;
                val
            }
        };

        for planet in orbits {
            if seen_set.insert(planet.to_string()) {
                stack.push(planet.to_string());
            }
        }
    }
    // ignore the COM
    // println!("{:?}", depth_map);
    // counter - 1
    depth_map.iter().fold(0, |acc, (key, val)| {
        acc + val
    })
}

pub fn rec_find_ancestors(current_planet: String, orbit_map: &HashMap<String, Vec<String>>, target_planet: String, mut complete: bool, result_vec: &mut Vec<String>) {

    if current_planet == target_planet {
        result_vec.push(current_planet.to_string());
        complete = true;
    }

    if !complete {
        let orbiters = orbit_map.get(&current_planet);
        result_vec.push(current_planet.to_string());

        match orbiters {
            None => { result_vec.pop().unwrap(); }
            Some(orbiters) => {
                for planet in orbiters {
                    rec_find_ancestors(planet.clone(), orbit_map, target_planet.clone(), complete, result_vec);
                }
                if *result_vec.last().unwrap() != target_planet {result_vec.pop().unwrap();}
            }
        };
    }
}

fn find_common_ancestor(vec1: Vec<String>, vec2: Vec<String>) -> String {
    let mut last_matching_ancestor = "".to_string();
    // println!("{vec1:?}");
    // println!("{vec2:?}");
    for (i, planet) in vec1.iter().enumerate() {
        if *planet != vec2[i] {
            break;
        } else {
            last_matching_ancestor = planet.to_string();
        }
    }
    last_matching_ancestor
}

fn find_closest_ancestor_a_and_b(orbit_map: &HashMap<String, Vec<String>>, start: String, a: String, b: String) -> String {
    let mut result_vec1: Vec<String> = vec![];
    let mut result_vec2: Vec<String> = vec![];

    rec_find_ancestors(start.clone(), &orbit_map, a, false, &mut result_vec1);
    rec_find_ancestors(start.clone(), &orbit_map, b, false, &mut result_vec2);
    find_common_ancestor(result_vec1, result_vec2)
}

pub fn find_number_of_jumps(orbit_map: &HashMap<String, Vec<String>>, start: String, a: String, b: String) -> i32{
    let ancestor = find_closest_ancestor_a_and_b(orbit_map, start, a.clone(), b.clone());
    let mut a_len_vec = vec![];
    let mut b_len_vec = vec![];
    rec_find_ancestors(ancestor.clone(), orbit_map, a.clone(), false, &mut a_len_vec);
    rec_find_ancestors(ancestor.clone(), orbit_map, b.clone(), false, &mut b_len_vec);
    // -1 since the ancestors vector includes the endpoint, -2 because we want to orbit what santa is orbiting
    a_len_vec.len() as i32 -2 + b_len_vec.len() as i32 -2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_str_vec<'a>() -> Vec<&'a str> {
        vec![
            "A)B",
            "B)C",
            "B)G",
            "B)H",
            "C)D",
            "D)E",
            "D)F",
        ]
    }

    // #[test]
    fn test_parser() {
        let pairs_vec = parse_pairs(&create_str_vec());
        let result_pairs = vec![OrbitPair::new("A", "B"), OrbitPair::new("B", "C"), OrbitPair::new("C", "D"), OrbitPair::new("D", "E")];
        assert_eq!(pairs_vec, result_pairs);
    }
    // #[test]
    fn test_insert() {
        // let root = OrbitTree::new("A");
        let pairs_vec = parse_pairs(&create_str_vec());

        let mut orbit_map = insert_orbits_to_hashmap(pairs_vec);
        let mut result_map: HashMap<String, Vec<String>> = HashMap::new();
        // {"D": ["E"], "C": ["D"], "B": ["C"], "A": ["B"]}
        result_map.insert("A".to_string(), vec!["B".to_string()]);
        result_map.insert("B".to_string(), vec!["C".to_string()]);
        result_map.insert("C".to_string(), vec!["D".to_string()]);
        result_map.insert("D".to_string(), vec!["E".to_string()]);
        assert_eq!(orbit_map, result_map);
        // println!("{:?}", orbit_map);

    }

    #[test]
    fn test_find_com() {
        let mut orbit_map = insert_orbits_to_hashmap(parse_pairs(&create_str_vec()));

        assert_eq!(find_leftmost_com(&orbit_map), Ok("A".to_string()));
    }

    #[test]
    fn test_count_orbits() {
        let mut orbit_map = insert_orbits_to_hashmap(parse_pairs(&create_str_vec()));


        let count = count_orbits(orbit_map);
        assert_eq!(18, count);
    }
    // find the closest common ancestor
    // find the depth between that ancestor and both of the endpoints
    #[test]
    fn test_find_ancestors() {
        let mut orbit_map = insert_orbits_to_hashmap(parse_pairs(&create_str_vec()));
        let mut result_vec: Vec<String> = vec![];

        rec_find_ancestors("A".to_string(), &orbit_map, "D".to_string(), false, &mut result_vec);
        assert_eq!(vec!["A", "B", "C", "D"],result_vec);
    }
    #[test]
    fn test_find_closest_common_ancestor() {
        let mut orbit_map = insert_orbits_to_hashmap(parse_pairs(&create_str_vec()));
        let mut result_vec1: Vec<String> = vec![];
        let mut result_vec2: Vec<String> = vec![];

        rec_find_ancestors("A".to_string(), &orbit_map, "D".to_string(), false, &mut result_vec1);
        rec_find_ancestors("A".to_string(), &orbit_map, "G".to_string(), false, &mut result_vec2);
        // println!("{result_vec1:?}");
        // println!("{result_vec2:?}");

        assert_eq!("B", find_common_ancestor(result_vec1, result_vec2));
    }
    #[test]
    fn test_ancestor_to_a_and_b() {
        let mut orbit_map = insert_orbits_to_hashmap(parse_pairs(&create_str_vec()));
        assert_eq!("B", find_closest_ancestor_a_and_b(&orbit_map, "A".to_string(), "F".to_string(), "H".to_string()));

    }

    #[test]
    fn test_find_number_of_jumps() {
        let mut orbit_map = insert_orbits_to_hashmap(parse_pairs(&create_str_vec()));
        assert_eq!(0, find_number_of_jumps(&orbit_map, "A".to_string(), "F".to_string(), "E".to_string()));
        assert_eq!(2, find_number_of_jumps(&orbit_map, "A".to_string(), "F".to_string(), "H".to_string()));

    }
}

