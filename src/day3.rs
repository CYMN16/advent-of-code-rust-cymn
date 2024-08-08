use std::collections::HashMap;

pub fn wire_crossing_manhattan(vec1: Vec<String>, vec2: Vec<String>) -> Result<i32, String> {
    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();
    let (mut x, mut y): (i32, i32) = (0, 0);
    for code in vec1 {
        //println!("code: {}", code);
        let (direction, move_length) = code.split_at(1);
        let move_length = move_length.parse::<i32>().unwrap();
        let dir: (i32, i32) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => {
                return Err("direction not found".to_string());
            }
        };
        for _i in 0..move_length {
            x += dir.0;
            y += dir.1;
            //grid.entry((x,y), true);
            grid.insert((x, y), true);
        }
    }
    //println!("grid len: {:?}", grid.len());
    let mut result_vec: Vec<(i32, i32)> = vec![];
    let (mut x, mut y): (i32, i32) = (0, 0);
    for code in vec2 {
        //println!("code: {}", code);
        let (direction, move_length) = code.split_at(1);
        let move_length = move_length.parse::<i32>().unwrap();
        let dir: (i32, i32) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => {
                return Err("direction not found".to_string());
            }
        };
        for _i in 0..move_length {
            x += dir.0;
            y += dir.1;
            if grid.contains_key(&(x, y)) {
                result_vec.push((x, y));
            }
        }
    }
    let mut distances_vec: Vec<i32> = vec![];
    for (a, b) in result_vec {
        distances_vec.push(manhattan_distance((a, b), (0, 0)));
    }
    distances_vec.sort();
    //println!("{:?}", distances_vec[0]);

    Ok(distances_vec[0])
}

fn manhattan_distance((a, b): (i32, i32), (c, d): (i32, i32)) -> i32 {
    (d - b).abs() + (c - a).abs()
}

pub fn wire_crossing_steps(vec1: Vec<String>, vec2: Vec<String>) -> Result<i32, String> {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut steps = 0;
    for code in vec1 {
        //println!("code: {}", code);
        let (direction, move_length) = code.split_at(1);
        let move_length = move_length.parse::<i32>().unwrap();
        let dir: (i32, i32) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => {
                return Err("direction not found".to_string());
            }
        };
        for _i in 0..move_length {
            steps += 1;
            x += dir.0;
            y += dir.1;
            //grid.entry((x,y), true);
            grid.insert((x, y), steps);
        }
    }
    //println!("grid len: {:?}", grid.len());
    let mut steps = 0;
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut total_steps_vec: Vec<i32> = vec![];
    for code in vec2 {
        //println!("code: {}", code);
        let (direction, move_length) = code.split_at(1);
        let move_length = move_length.parse::<i32>().unwrap();
        let dir: (i32, i32) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => {
                return Err("direction not found".to_string());
            }
        };
        for _i in 0..move_length {
            steps += 1;
            x += dir.0;
            y += dir.1;
            if let Some(other_steps) = grid.get(&(x, y)) {
                total_steps_vec.push(other_steps + steps);
            }
            //match grid.get(&(x, y)) {
            //    Some(other_steps) => {
            //        total_steps_vec.push(other_steps + steps);
            //    }
            //    None => {}
            //}
        }
    }

    total_steps_vec.sort();

    Ok(total_steps_vec[0])
}
