use std::{io, num::ParseIntError};

#[derive(Debug, PartialEq)]
pub enum ParamMode {
    Positional,
    Immediate,
}
#[derive(Debug)]
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

pub fn parse_instruction(instruction: i32) -> (Opcode, [ParamMode; 3]) {
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

fn user_input() -> Result<i32, ParseIntError> {
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
        println!("{:?}", vec[instruction_pointer]);
        let (opcode, param_modes) = parse_instruction(vec[instruction_pointer]);
        

        //println!("{}", instruction_pointer);
        //println!("{:?}", opcode);
        //println!("{:?}", param_modes);
        match opcode {
            Opcode::Sum => {
                let param1 = match param_modes[0] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 1].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 1],
                };
                let param2 = match param_modes[1] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 2].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 2],
                };
                let par3_index = vec[instruction_pointer + 3].clone() as usize;
                let param3 = match param_modes[2] {
                    ParamMode::Positional => &mut vec[par3_index],
                    ParamMode::Immediate => &mut vec[instruction_pointer + 3],
                };
                *param3 = param1 + param2;
                instruction_pointer += 4;
            }
            Opcode::Multiply => {
                let param1 = match param_modes[0] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 1].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 1],
                };
                let param2 = match param_modes[1] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 2].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 2],
                };
                let par3_index = vec[instruction_pointer + 3].clone() as usize;
                let param3 = match param_modes[2] {
                    ParamMode::Positional => &mut vec[par3_index],
                    ParamMode::Immediate => &mut vec[instruction_pointer + 3],
                };
                *param3 = param1 * param2;
                instruction_pointer += 4;
            }
            Opcode::Input => {
                let par1_index = vec[instruction_pointer + 1].clone() as usize;
                let param1 = match param_modes[0] {
                    ParamMode::Positional => &mut vec[par1_index],
                    ParamMode::Immediate => &mut vec[instruction_pointer + 1],
                };
                println!("Please enter an input instruction: ");
                *param1 = user_input().unwrap();
                instruction_pointer += 2;
            }
            Opcode::Output => {
                let par1_index = vec[instruction_pointer + 1].clone() as usize;
                let param1 = match param_modes[0] {
                    ParamMode::Positional => vec[par1_index],
                    ParamMode::Immediate => vec[instruction_pointer + 1],
                };

                result = param1;
                println!("OUTPUT: {}", result);
                instruction_pointer += 2;
            }
            Opcode::JumpIfTrue => {
                let param1 = match param_modes[0] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 1].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 1],
                };
                let param2 = match param_modes[1] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 2].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 2],
                };
                if param1 != 0 {
                    instruction_pointer = param2 as usize
                } else {
                    instruction_pointer += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let param1 = match param_modes[0] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 1].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 1],
                };
                let param2 = match param_modes[1] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 2].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 2],
                };
                if param1 == 0 {
                    instruction_pointer = param2 as usize
                } else {
                    instruction_pointer += 3;
                }
            }
            Opcode::LessThan => {
                let param1 = match param_modes[0] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 1].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 1],
                };
                let param2 = match param_modes[1] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 2].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 2],
                };
                let par3_index = vec[instruction_pointer + 3].clone() as usize;
                let param3 = match param_modes[2] {
                    ParamMode::Positional => &mut vec[par3_index],
                    ParamMode::Immediate => &mut vec[instruction_pointer + 3],
                };
                if param1 < param2 {
                    *param3 = 1;
                } else {
                    *param3 = 0;
                }
                instruction_pointer += 4;
            }

            Opcode::Equals => {
                let param1 = match param_modes[0] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 1].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 1],
                };
                let param2 = match param_modes[1] {
                    ParamMode::Positional => vec[vec[instruction_pointer + 2].clone() as usize],
                    ParamMode::Immediate => vec[instruction_pointer + 2],
                };
                let par3_index = vec[instruction_pointer + 3].clone() as usize;
                let param3 = match param_modes[2] {
                    ParamMode::Positional => &mut vec[par3_index],
                    ParamMode::Immediate => &mut vec[instruction_pointer + 3],
                };

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

fn computer(mut vec: Vec<u32>) -> Vec<u32> {
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
            _ => {
                eprintln!("Unmatched opcode!");
            }
        };
        index += 4;
    }
    vec
}

fn pair_computer(vec: Vec<u32>, result: u32) -> Result<(u32, u32), String> {
    for i in 0..=99 {
        for j in 0..=99 {
            let mut vec_clone = vec.clone();
            vec_clone[1] = i;
            vec_clone[2] = j;
            let res = computer(vec_clone)[0];
            if res == result {
                return Ok((i, j));
            };
        }
    }
    Err("No pairs satisfy the result".to_string())
}
