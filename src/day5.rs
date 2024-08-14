use std::{io, num::ParseIntError};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParamMode {
    Positional,
    Immediate,
}
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Sum = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Stop = 99,
}

impl TryFrom<u8> for ParamMode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Positional),
            1 => Ok(Self::Immediate),
            _ => Err("ParamMode not matched!".to_string()),
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Sum),
            2 => Ok(Self::Multiply),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            99 => Ok(Self::Stop),
            _ => Err(format!("{} Opcode not matched", value)),
        }
    }
}

pub(crate) fn parse_instruction(instruction: i32) -> (Opcode, [ParamMode; 3]) {
    let instruction_str = format!("{:05}", instruction);
    let opcode: Opcode =
        Opcode::try_from(instruction_str[3..].to_string().parse::<u8>().unwrap()).unwrap();
    let param1 =
        ParamMode::try_from(instruction_str[2..3].to_string().parse::<u8>().unwrap()).unwrap();
    let param2 =
        ParamMode::try_from(instruction_str[1..2].to_string().parse::<u8>().unwrap()).unwrap();
    let param3 =
        ParamMode::try_from(instruction_str[0..1].to_string().parse::<u8>().unwrap()).unwrap();
    (opcode, [param1, param2, param3])
}

pub(crate) fn get_param(vec: &Vec<i32>, index: usize, param_mode: ParamMode) -> i32 {
    match param_mode {
        ParamMode::Positional => { vec[vec[index] as usize] }
        ParamMode::Immediate => { vec[index] }
    }
}
pub(crate) fn get_mut_param(vec: &mut Vec<i32>, index: usize, param_mode: ParamMode) -> &mut i32 {
    let pos_index = vec[index] as usize;
    match param_mode {
        ParamMode::Positional => { &mut vec[pos_index] }
        ParamMode::Immediate => { &mut vec[index] }
    }
}

pub(crate) fn user_input() -> Result<i32, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input.trim().parse::<i32>()
}

pub fn computer_ver2(mut vec: Vec<i32>) -> i32 {
    let mut instruction_pointer = 0;
    let mut result = 0;
    loop {
        let (opcode, param_modes) = parse_instruction(vec[instruction_pointer]);

        match opcode {
            Opcode::Sum => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, param_modes[1]);
                let param3 = get_mut_param(&mut vec, instruction_pointer + 3, param_modes[2]);
                *param3 = param1 + param2;
                instruction_pointer += 4;
            }
            Opcode::Multiply => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, param_modes[1]);
                let param3 = get_mut_param(&mut vec, instruction_pointer + 3, param_modes[2]);
                *param3 = param1 * param2;
                instruction_pointer += 4;
            }
            Opcode::Input => {
                let param1 = get_mut_param(&mut vec, instruction_pointer + 1, param_modes[0]);

                println!("Please enter an input instruction: ");
                *param1 = user_input().unwrap();
                instruction_pointer += 2;
            }
            Opcode::Output => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                result = param1;
                println!("OUTPUT: {}", result);
                instruction_pointer += 2;
            }
            Opcode::JumpIfTrue => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, param_modes[1]);
                if param1 != 0 {
                    instruction_pointer = param2 as usize
                } else {
                    instruction_pointer += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, param_modes[1]);
                if param1 == 0 {
                    instruction_pointer = param2 as usize
                } else {
                    instruction_pointer += 3;
                }
            }
            Opcode::LessThan => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, param_modes[1]);
                let param3 = get_mut_param(&mut vec, instruction_pointer + 3, param_modes[2]);
                if param1 < param2 {
                    *param3 = 1;
                } else {
                    *param3 = 0;
                }
                instruction_pointer += 4;
            }

            Opcode::Equals => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, param_modes[1]);
                let param3 = get_mut_param(&mut vec, instruction_pointer + 3, param_modes[2]);

                if param1 == param2 {
                    *param3 = 1;
                } else {
                    *param3 = 0;
                }
                instruction_pointer += 4;
            }
            Opcode::Stop => {
                break;
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    fn create_vec() -> Vec<i32> {
        vec![1, 2, 3, 4, 5, 6, 7, 8]
    }

    #[test]
    fn param_separation() {
        let mut input_vec = create_vec();

        assert_eq!(get_param(&input_vec, 1, ParamMode::Immediate), 2);
    }
    #[test]
    fn mut_param_separation() {
        let mut input_vec = create_vec();
        let ptr = &mut input_vec[1];
        let num = get_mut_param(&mut input_vec, 1, ParamMode::Immediate);
        *num = 99;
        assert_eq!(input_vec[1], 99);

        println!("{:?}", input_vec)
    }
}
