use std::time::Duration;
use std::collections::BTreeSet;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError, RecvTimeoutError};
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParamMode {
    Positional,
    Immediate,
    Relative,
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
    RelativeBaseOffset = 9,
    Stop = 99,
}

impl TryFrom<u8> for ParamMode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Positional),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
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
            9 => Ok(Self::RelativeBaseOffset),
            99 => Ok(Self::Stop),
            _ => Err(format!("{} Opcode not matched", value)),
        }
    }
}
fn get_param(vec: &Vec<i64>, index: usize, relative_base: i64, param_mode: ParamMode) -> i64 {
    match param_mode {
        ParamMode::Positional => { vec[vec[index] as usize] }
        ParamMode::Immediate => { vec[index] }
        ParamMode::Relative => { vec[(relative_base + vec[index]) as usize] }
    }
}

fn get_mut_param(vec: &mut Vec<i64>, index: usize, relative_base: i64, param_mode: ParamMode) -> &mut i64 {
    let pos_index = vec[index];
    match param_mode {
        ParamMode::Positional => { &mut vec[pos_index as usize] }
        ParamMode::Immediate => { &mut vec[index] }
        ParamMode::Relative => { &mut vec[(relative_base + pos_index) as usize] }
    }
}
fn parse_instruction(instruction: i64) -> (Opcode, [ParamMode; 3]) {
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

//Threaded
pub fn computer_ver5(mut program: Vec<i64>, sender: SyncSender<i64>, receiver: Receiver<i64>) -> i64 {
    let mut vec = program.clone();
    vec.append(&mut Vec::from([0; 16000]));

    // println!("{}", vec.len());
    let mut instruction_pointer = 0;
    let mut relative_base: i64 = 0;
    let mut result = 0;
    loop {
        let (opcode, param_modes) = parse_instruction(vec[instruction_pointer]);

        match opcode {
            Opcode::Sum => {
                let param1 = get_param(&vec, instruction_pointer + 1, relative_base, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, relative_base, param_modes[1]);
                let param3 = get_mut_param(&mut vec, instruction_pointer + 3, relative_base, param_modes[2]);
                *param3 = param1 + param2;
                instruction_pointer += 4;
            }
            Opcode::Multiply => {
                let param1 = get_param(&vec, instruction_pointer + 1, relative_base, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, relative_base, param_modes[1]);
                let param3 = get_mut_param(&mut vec, instruction_pointer + 3, relative_base, param_modes[2]);
                *param3 = param1 * param2;
                instruction_pointer += 4;
            }
            Opcode::Input => {
                let param1 = get_mut_param(&mut vec, instruction_pointer + 1, relative_base, param_modes[0]);
                *param1 = receiver.recv().expect("Did not receive any input!");
                println!("received input: {param1:?}");
                // println!("Received {param1} input!");
                instruction_pointer += 2;
            }
            Opcode::Output => {
                let param1 = get_param(&vec, instruction_pointer + 1, relative_base, param_modes[0]);
                match sender.try_send(param1) {
                    Ok(_) => {}
                    Err(TrySendError::Disconnected(_)) => { result = param1 }
                    Err(e) => { eprintln!("Received {e} error!") }
                };
                instruction_pointer += 2;
            }
            Opcode::JumpIfTrue => {
                let param1 = get_param(&vec, instruction_pointer + 1, relative_base, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, relative_base, param_modes[1]);
                if param1 != 0 {
                    instruction_pointer = param2 as usize
                } else {
                    instruction_pointer += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let param1 = get_param(&vec, instruction_pointer + 1, relative_base, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, relative_base, param_modes[1]);
                if param1 == 0 {
                    instruction_pointer = param2 as usize
                } else {
                    instruction_pointer += 3;
                }
            }
            Opcode::LessThan => {
                let param1 = get_param(&vec, instruction_pointer + 1, relative_base, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, relative_base, param_modes[1]);
                let param3 = get_mut_param(&mut vec, instruction_pointer + 3, relative_base, param_modes[2]);
                if param1 < param2 {
                    *param3 = 1;
                } else {
                    *param3 = 0;
                }
                instruction_pointer += 4;
            }

            Opcode::Equals => {
                let param1 = get_param(&vec, instruction_pointer + 1, relative_base, param_modes[0]);
                let param2 = get_param(&vec, instruction_pointer + 2, relative_base, param_modes[1]);
                let param3 = get_mut_param(&mut vec, instruction_pointer + 3, relative_base, param_modes[2]);

                if param1 == param2 {
                    *param3 = 1;
                } else {
                    *param3 = 0;
                }
                instruction_pointer += 4;
            }
            Opcode::RelativeBaseOffset => {
                let param1 = get_param(&vec, instruction_pointer + 1, relative_base, param_modes[0]);
                relative_base += param1;
                instruction_pointer += 2;
            }
            Opcode::Stop => {
                break;
            }
        }
    }
    result
}

pub fn run_computer(vec: Vec<i64>, input: i64) -> i64 {
    let (sender1, receiver2) = sync_channel(10);
    let (sender2, receiver1) = sync_channel(10);

    sender2.send(input).unwrap();
    let handle = thread::spawn(|| { computer_ver5(vec, sender1, receiver1) });
    // let output_res = match receiver2.recv_timeout(Duration::from_millis(5000)) {
    //     Ok(val) => {val}
    //     Err(e) => {eprintln!("{}", e); -1}
    // };

    let mut outputs = vec![];

    let collector = thread::spawn(move || {
        loop {
            match receiver2.recv_timeout(Duration::from_millis(800)) {
                Ok(val) => { outputs.push(val) }
                Err(_) => { break }
            }
        }
        outputs
    });


    let res = handle.join();
    let output_res = collector.join().unwrap();
    // println!("{:?}", res);
    println!("{:?}", output_res);
    res.unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    fn create_vec() -> Vec<i64> {
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    }
    /*
    #[test]
    fn test_computer_ver5() {
        let vec = create_vec();
        let (sender1, receiver2) = sync_channel(5);
        let (sender2, receiver1) = sync_channel(5);

        let handle = thread::spawn(|| { computer_ver5(vec, sender1, receiver1) });

        let mut outputs = vec![];

        let collector = thread::spawn(move || {
            loop {
                match receiver2.recv_timeout(Duration::from_millis(800)) {
                    Ok(val) => { outputs.push(val) }
                    Err(_) => { break }
                }
            }
            outputs
        });

        // sender2.send(1).unwrap();
        // let out = receiver2.recv().unwrap();

        let res = handle.join();
        let output_res = collector.join().unwrap();
        println!("{:?}", res);
        println!("{:?}", output_res);
    }
    */
    #[test]
    fn test_run() {
        let vec = create_vec();
        println!("{}", run_computer(vec, 1));
    }
}
