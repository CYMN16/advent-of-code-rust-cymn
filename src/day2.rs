
pub fn computer(mut vec: Vec<u32>) -> Vec<u32> {
    let mut index = 0;

    loop {
        match vec[index] {
            1 => {
                //println!("Inside Add");
                let i1 = vec[index + 1] as usize;
                let i2 = vec[index + 2] as usize;
                let i3 = vec[index + 3] as usize;
                vec[i3] = vec[i1] + vec[i2];
            }
            2 => {
                //println!("Inside Mult");
                let i1 = vec[index + 1] as usize;
                let i2 = vec[index + 2] as usize;
                let i3 = vec[index + 3] as usize;
                vec[i3] = vec[i1] * vec[i2];
            }
            99 => {
                //println!("Inside Stop");
                break;
            }
            _ => {eprintln!("Unmatched opcode!");}
        };
        index += 4;
    }
    vec
}

pub fn pair_computer(vec: Vec<u32>, result: u32) -> Result<(u32,u32), String>{
    for i in 0..=99{
        for j in 0..=99{
            let mut vec_clone = vec.clone();
            vec_clone[1] = i;
            vec_clone[2] = j;
            let res = computer(vec_clone)[0];
            if res == result {return Ok((i,j))};
        }
    }
    Err("No pairs satisfy the result".to_string())
    
}
