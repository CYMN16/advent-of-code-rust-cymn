pub fn fuel_req(input: i32) -> i32 {
    input / 3 - 2
}

pub fn fuel_req_recursive(input: i32) -> i32 {
    if input > 6 {
        let input = input / 3 - 2;
        input + fuel_req_recursive(input)
    } else {
        0
    }
}

//(200/3 - 2) + (200/3 - 2) / 3 - 2
