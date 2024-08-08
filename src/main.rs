#![allow(unused_mut, unused_variables)]

use std::fs;

use day5::computer_ver2;


//use day1::{fuel_req, fuel_req_recursive};
//use day2::{computer, pair_computer};
//use day3::{wire_crossing_manhattan, wire_crossing_steps};
//use day4::{extra_num_possible_passwords_for_container, num_possible_passwords_for_container};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
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
    

    */
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

    

}
