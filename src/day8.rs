use std::collections::{BTreeSet, HashMap};

fn split_into_layers(image: String, m: usize, n: usize) -> Vec<String> {
    let mut image_str = image.clone();
    let mut result = vec![];
    while image_str.len() > 0 {
        // println!("{image_str}");
        let rest = image_str.split_off(m * n);
        let slice = image_str.clone();

        image_str = rest;
        result.push(slice);
    }
    result
}

fn count_0s(str: &str) -> usize {
    let mut count = 0;
    for c in str.chars() {
        if c == '0' { count += 1 };
    }
    count
}

fn count_chars(str: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for c in str.chars() {
        *result.entry(c).or_insert(0) += 1;
    }
    result
}

pub fn find_smallest0_mult1_2(str: &str, m: usize, n: usize) -> u32 {
    let (mut smallest_0map, mut count) = (HashMap::new(), m * n);
    let layers = split_into_layers(str.to_string(), m, n);

    for (index, layer) in layers.iter().enumerate() {
        let map = count_chars(&layer);
        let cur_count = *map.get(&'0').unwrap();
        if cur_count < count {
            smallest_0map = map;
            count = cur_count
        }
    }

    println!("{:?}", smallest_0map);
    let c1: usize = match smallest_0map.get(&'1') {
        None => { 0 }
        Some(n) => { *n }
    };
    let c2: usize = match smallest_0map.get(&'2') {
        None => { 0 }
        Some(n) => { *n }
    };

    c1 as u32 * c2 as u32
}

fn decode_pixels(str: Vec<String>) -> HashMap<usize, char> {
    let mut map = HashMap::new();
    for layer in str {
        for (index, c) in layer.chars().enumerate() {
            if !map.contains_key(&index) {
                if c == '1' || c == '0' {
                    map.insert(index, c);
                }
            };
        }
    }
    println!("{:?}", map);
    map
}

pub fn assemble_image(image: String, m: usize, n: usize) -> Vec<String> {
    let str = split_into_layers(image, m, n);
    let map = decode_pixels(str);
    let mut image_vec = vec![];
    for i in 0..n {
        let mut layer = "".to_string();
        for j in 0..m {
            let index = i * m + j;
            println!("{}", index);
            match map.get(&index) {
                None => { layer.push('2') }
                Some(c) => { layer.push(*c) }
            }
        }
        image_vec.push(layer);
    }
    image_vec
}
#[cfg(test)]
mod tests {
    use super::*;

    fn create_str() -> String {
        String::from(
            "120222\
                200021\
                121021")
    }
    #[test]
    fn test_layering() {
        // m x n pixels on each layer, split each layer
        let str = create_str();

        println!("{:?}", split_into_layers(str, 3, 2));
    }
    #[test]
    fn test_0counting() {
        let str = create_str();
        for (index, layer) in split_into_layers(str, 3, 2).iter().enumerate() {
            println!("index: {index}, count: {:?}", count_0s(&layer));
        }
    }

    #[test]
    fn test_count_chars() {
        let str = create_str();
        println!("{:?}", count_chars(&str));
    }

    #[test]
    fn test_find_smallest0_mult1_2() {
        let str = create_str();
        println!("{}", find_smallest0_mult1_2(&str, 3, 2));
    }

    #[test]
    fn test_decode_pixels() {
        let str = create_str();
        println!("{:?}", decode_pixels(split_into_layers(str, 3, 2)));
    }
    #[test]
    fn test_assemble_image() {
        let str = create_str();
        println!("{:?}", assemble_image(str, 3, 2));
    }

    #[test]
    fn test_custom() {
        let str = "0222112222120000".to_string();
        println!("{:?}", assemble_image(str, 2,2));
    }
}
