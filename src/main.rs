#![allow(unused_mut, unused_variables, dead_code, unused_imports)]

use std::fs;
use crate::day10::{destroy_asteroids, find_best_location};
use crate::day12::{calculate_repeating_index, calculate_repetition_period, calculate_total_energy, full_steps_energy_period};
use crate::day14::{Inventory, RecipeBook};
use crate::day7::{threaded_feedback_loop_phase_combinations, try_phase_combinations};
use crate::day8::{assemble_image, find_smallest0_mult1_2};
// use crate::day9::run_computer;
// use crate::day6::{count_orbits, find_number_of_jumps, insert_orbits_to_hashmap, parse_pairs};
//use day1::{fuel_req, fuel_req_recursive};
//use day2::{computer, pair_computer};
//use day3::{wire_crossing_manhattan, wire_crossing_steps};
//use day4::{extra_num_possible_passwords_for_container, num_possible_passwords_for_container};
// use day5::computer_ver2;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn main() {
    /*
    let inputs = fs::read("./inputs/day1").unwrap();
    let inputs_vec: Vec<i32> = String::from_utf8(inputs)
        .unwrap()
        .split_terminator('\n')
        .map(|item| {
            if !item.is_empty() {
                item.parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .collect();
    let res1 = inputs_vec.iter().fold(0, |acc, item| acc + fuel_req(*item));
    println!("Day - 1");
    println!("sum of required fuel by all modules: {}", res1);
    let res2 = inputs_vec
        .iter()
        .fold(0, |acc, item| acc + fuel_req_recursive(*item));
    println!("sum of recursive required fuel by all modules: {}", res2);

    println!("Day - 2");
    let inputs = fs::read("./inputs/day2").unwrap();
    let mut inputs_vec: Vec<u32> = String::from_utf8(inputs)
        .unwrap()
        .split_terminator(&[',', '\n'][..])
        .map(|item| {
            if !item.is_empty() {
                item.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .collect();
    inputs_vec[1] = 12;
    inputs_vec[2] = 2;
    println!("{:?}", inputs_vec);

    let res3 = computer(inputs_vec.clone());
    println!("result returned from the computer: {:?}", res3[0]);
    let expected_result = 19690720;
    let (noun, verb) = pair_computer(inputs_vec, expected_result).unwrap();
    println!(
        "returned pair for: {}, is {:?}",
        expected_result,
        (noun, verb)
    );
    println!(
        "result of the expression: 100 * noun + verb = {}",
        (100 * noun + verb)
    );

    println!("Day - 3");
    let inputs = fs::read("./inputs/day3").unwrap();
    let inputs_vec: Vec<String> = String::from_utf8(inputs)
        .unwrap()
        .split_terminator('\n')
        .map(|item| {
            if !item.is_empty() {
                item.to_string()
            } else {
                "".to_string()
            }
        })
        .collect();
    let vec1: Vec<String> = inputs_vec[0]
        .split_terminator(',')
        .map(|item| {
            if !item.is_empty() {
                item.to_string()
            } else {
                "".to_string()
            }
        })
        .collect();
    let vec2: Vec<String> = inputs_vec[1]
        .split_terminator(',')
        .map(|item| {
            if !item.is_empty() {
                item.to_string()
            } else {
                "".to_string()
            }
        })
        .collect();
    println!("shortest manhattan distance: {:?}", wire_crossing_manhattan(vec1.clone(), vec2.clone()).unwrap());
    println!("shortest steps: {:?}", wire_crossing_steps(vec1, vec2).unwrap());
    println!("Day - 4");

    let range = 193651..649729;
    println!("number of passwords for the range {:?} : {:?}", range.clone(), num_possible_passwords_for_container(range.clone()));
    println!("number of passwords for the range with extra steps {:?} : {:?}", range.clone(), extra_num_possible_passwords_for_container(range.clone()));
    

    println!("Day - 5");
    let inputs = fs::read("./inputs/day5").unwrap();
    let mut inputs_vec: Vec<i32> = String::from_utf8(inputs)
        .unwrap()
        .split_terminator(&[',', '\n'][..])
        .map(|item| {
            if !item.is_empty() {
                item.parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .collect();

    //println!("{:?}", inputs_vec);
    println!("Result from computer: {}",computer_ver2(inputs_vec));
    println!("Day - 6");

    let inputs = fs::read("./inputs/day6").unwrap();
    let binding = String::from_utf8(inputs)
        .unwrap();
    let mut inputs_vec: Vec<&str> = binding
        .split_terminator("\r\n")
        .map(|item| {
            if !item.is_empty() {
                item
            } else {
                ""
            }
        }).collect();
    let pairs_vec = parse_pairs(&inputs_vec);
    let orbit_map = insert_orbits_to_hashmap(pairs_vec);

    // let com = find_leftmost_com(&orbit_map).unwrap();
    println!("number of connections: {}", count_orbits(orbit_map.clone()));

    println!("number of jumps for SAN to YOU: {}", find_number_of_jumps(&orbit_map, "COM".to_string(), "SAN".to_string(), "YOU".to_string()))
    println!("Day - 7");


    let inputs = fs::read("./inputs/day7").unwrap();
    let mut inputs_vec: Vec<i32> = String::from_utf8(inputs)
        .unwrap()
        .split_terminator(&[',', '\n'][..])
        .map(|item| {
            if !item.is_empty() {
                item.parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .collect();

    let set = vec![0, 1, 2, 3, 4];
    println!("{}", try_phase_combinations(inputs_vec.clone(), set.clone()));
    let set = vec![5, 6, 7, 8, 9];
    println!("{}", threaded_feedback_loop_phase_combinations(inputs_vec.clone(), set));

    println!("Day - 8");

    let inputs = fs::read("./inputs/day8").unwrap();
    let input_str = String::from_utf8(inputs).unwrap();

    println!("{}",find_smallest0_mult1_2(&input_str, 25, 6));
    println!("{:?}",assemble_image(input_str.clone(), 25, 6));


    println!("Day - 9");
    let inputs = fs::read("./inputs/day9").unwrap();
    let mut inputs_vec: Vec<i64> = String::from_utf8(inputs)
        .unwrap()
        .split_terminator(&[',', '\n'][..])
        .map(|item| {
            if !item.is_empty() {
                item.parse::<i64>().unwrap()
            } else {
                0
            }
        })
        .collect();

    println!("result: {:?}", run_computer(inputs_vec.clone(), 1));
    println!("result: {:?}", run_computer(inputs_vec.clone(), 2));


    println!("Day - 10");
    let inputs = fs::read("./inputs/day10").unwrap();
    let input_str: Vec<Vec<char>> = String::from_utf8(inputs).unwrap().split('\n').map(|item| {
        if !item.is_empty() {
            item.trim().chars().collect()
        } else {
            "".chars().collect()
        }
    }).collect();

    let ((i,j), max_asteroids) = find_best_location(input_str.clone());
    println!("{:?}", ((i, j), max_asteroids));

    destroy_asteroids(input_str, (i,j));

    println!("Day - 11");
    let inputs = fs::read("./inputs/day11").unwrap();
    let mut inputs_vec: Vec<i64> = String::from_utf8(inputs)
        .unwrap()
        .split_terminator(&[',', '\n'][..])
        .map(|item| {
            if !item.is_empty() {
                item.parse::<i64>().unwrap()
            } else {
                0
            }
        })
        .collect();
    day11::run_computer(inputs_vec);
    println!("Day - 12");
    let inputs = fs::read("./inputs/day12").unwrap();

    let binding = String::from_utf8(inputs)
        .unwrap();
    let mut inputs_vec: Vec<&str> = binding
        .split_terminator("\r\n")
        .map(|item| {
            if !item.is_empty() {
                item
            } else {
                ""
            }
        })
        .collect();
    println!("{:?}", calculate_total_energy(inputs_vec.clone(), 1000));
    println!("{:?}", calculate_repetition_period(inputs_vec, 1000000));
    println!("Day - 13");
    let inputs = fs::read("./inputs/day13").unwrap();
    let mut inputs_vec: Vec<i64> = String::from_utf8(inputs)
        .unwrap()
        .split_terminator(&[',', '\n'][..])
        .map(|item| {
            if !item.is_empty() {
                item.parse::<i64>().unwrap()
            } else {
                0
            }
        })
        .collect();
    // day13::run_computer(inputs_vec);
    day13::run_computer_with_ruscii(inputs_vec);

     */

    println!("Day - 14");
    let inputs = fs::read("./inputs/day14").unwrap();

    let binding = String::from_utf8(inputs).unwrap();
    let recipe_book = RecipeBook::new(binding.as_str());
    let mut inventory = Inventory::new(recipe_book);
    // inventory.request_item("FUEL", 10);
    
    // let cost_per_fuel = inventory.get_used_ore();
    // println!("{:?}", cost_per_fuel);
    let trillion: i64 = 1_000_000_000_000;
    // inventory.craft_until_n("FUEL", trillion);
    println!("max fuel: {}", inventory.craft_until_n("FUEL", trillion));
    println!("{:?}", inventory.get_unused_ingredients());
    // println!("max craftable fuel: {}", inventory.get_max_fuel_craftable(trillion));
    

}
