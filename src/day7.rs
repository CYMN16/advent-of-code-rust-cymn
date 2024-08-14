use std::collections::BTreeSet;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::thread;
use crate::day5::*;

pub fn computer_ver3(mut vec: Vec<i32>, inputs: Vec<i32>, output: &mut i32) -> i32 {
    let mut instruction_pointer = 0;
    let mut result;
    let mut input_counter = 0;
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

                *param1 = inputs[input_counter as usize];
                input_counter += 1;
                instruction_pointer += 2;
            }
            Opcode::Output => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                *output = param1;
                // println!("OUTPUT: {}", result);
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
                result = 0;
                break;
            }
        }
    }
    result
}

fn phase_setter(mut vec: Vec<i32>, phase_setting: Vec<i32>) -> i32 {
    //also the first input of 0
    let mut last_result = 0;
    let mut output = 0;

    for i in 0..5 {
        // println!("{last_result}");
        computer_ver3(vec.clone(), vec![phase_setting[i], output], &mut output);
    }
    output
}

fn rec_get_permutations(generated_perms: &mut Vec<Vec<i32>>, current_perm: &mut Vec<i32>, mut elements_to_permute: BTreeSet<i32>) {
    if !elements_to_permute.is_empty() {
        for element in elements_to_permute.clone() {
            let next_perm = &mut current_perm.clone();
            next_perm.push(element.clone());
            let mut remaining_elements = elements_to_permute.clone();
            remaining_elements.remove(&element);
            rec_get_permutations(generated_perms, next_perm, remaining_elements);
        }
    } else {
        generated_perms.push(current_perm.clone());
    }
}
pub fn try_phase_combinations(mut vec: Vec<i32>, ints: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut set = BTreeSet::from_iter(ints);
    let mut perms = vec![];
    let mut cur_perm = vec![];
    rec_get_permutations(&mut perms, &mut cur_perm, set);


    for p in perms {
        let res = phase_setter(vec.clone(), p);
        // println!("{res}");
        if res > max { max = res };
    }
    max
}
fn phase_setter_feedback_loop(mut vec: Vec<i32>, phase_setting: Vec<i32>) -> i32 {
    //also the first input of 0
    let mut last_result = 0;
    let mut output = 0;

    loop {
        for i in 0..5 {
            last_result = computer_ver3(vec.clone(), vec![phase_setting[i], output.clone()], &mut output);
        }
        if last_result == 0 { break; }
    }
    output
}

//Threaded
pub fn computer_ver4(mut vec: Vec<i32>, sender: SyncSender<i32>, receiver: Receiver<i32>) -> i32 {
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

                *param1 = receiver.recv().expect("Did not receive any input!");
                // println!("Received {param1} input!");
                instruction_pointer += 2;
            }
            Opcode::Output => {
                let param1 = get_param(&vec, instruction_pointer + 1, param_modes[0]);
                match sender.try_send(param1) {
                    Ok(_) => {},
                    Err(TrySendError::Disconnected(i32)) => {result = param1}
                    Err(e) => {eprintln!("Received {e} error!")}
                };
                // println!("OUTPUT: {}", result);
                // result = param1;
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

fn threaded_phase_setter(mut vec: Vec<i32>, phase_setting: Vec<i32>) -> i32 {
    //also the first input of 0
    let mut last_result = 0;
    // let mut output = 0;

    let (amp1_send, amp2_recv) = sync_channel(5);
    let (amp2_send, amp3_recv) = sync_channel(5);
    let (amp3_send, amp4_recv) = sync_channel(5);
    let (amp4_send, amp5_recv) = sync_channel(5);
    let (amp5_send, amp1_recv) = sync_channel(5);

    let sender = amp1_send.clone();
    let cloned_vec = vec.clone();
    let amp1 = thread::spawn(move || { computer_ver4(cloned_vec, sender, amp1_recv) });
    let sender = amp2_send.clone();
    let cloned_vec = vec.clone();
    let amp2 = thread::spawn(move || { computer_ver4(cloned_vec, sender, amp2_recv) });
    let sender = amp3_send.clone();
    let cloned_vec = vec.clone();
    let amp3 = thread::spawn(move || { computer_ver4(cloned_vec, sender, amp3_recv) });
    let sender = amp4_send.clone();
    let cloned_vec = vec.clone();
    let amp4 = thread::spawn(move || { computer_ver4(cloned_vec, sender, amp4_recv) });
    let sender = amp5_send.clone();
    let cloned_vec = vec.clone();
    let amp5 = thread::spawn(move || { computer_ver4(cloned_vec, sender, amp5_recv) });

    amp5_send.send(phase_setting[0]).expect("Couldn't send phase setting!");
    amp5_send.send(0).expect("Couldn't send init val!");
    amp1_send.send(phase_setting[1]).expect("Couldn't send phase setting!");
    amp2_send.send(phase_setting[2]).expect("Couldn't send phase setting!");
    amp3_send.send(phase_setting[3]).expect("Couldn't send phase setting!");
    amp4_send.send(phase_setting[4]).expect("Couldn't send phase setting!");

    amp1.join().unwrap();
    amp2.join().unwrap();
    amp3.join().unwrap();
    amp4.join().unwrap();
    amp5.join().unwrap()
}

pub fn threaded_feedback_loop_phase_combinations(mut vec: Vec<i32>, ints: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut set = BTreeSet::from_iter(ints);
    let mut perms = vec![];
    let mut cur_perm = vec![];
    rec_get_permutations(&mut perms, &mut cur_perm, set);

    for p in perms {
        let res = threaded_phase_setter(vec.clone(), p);
        // println!("{res}");
        if res > max { max = res };
    }
    max
}
#[cfg(test)]
mod tests {
    use super::*;
    fn create_vec() -> Vec<i32> {
        vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]
    }

    #[test]
    fn test_amplifier_io() {
        let mut vec = create_vec();
        let phase_setting = vec![4, 3, 2, 1, 0];
        assert_eq!(43210, phase_setter(vec, phase_setting));
        let mut vec = vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
                           1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0];
        let phase_setting = vec![1, 0, 4, 3, 2];
        assert_eq!(65210, phase_setter(vec, phase_setting));
    }
    #[test]
    fn test_phase_perms() {
        let vec = vec![0, 1, 2, 3, 4];
        let mut result_vec = vec![];
        let mut cur_perm = vec![];
        rec_get_permutations(&mut result_vec, &mut cur_perm, BTreeSet::from_iter(vec));

        println!("{result_vec:?}");
    }
    #[test]
    fn test_max_from_phase_combinations() {
        let mut vec = vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
                           1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0];
        let set = vec![0, 1, 2, 3, 4];
        println!("{}", try_phase_combinations(vec, set));
    }
    #[test]
    fn test_thread_send() {

        let (amp1_send, amp2_recv) = sync_channel(5);
        let (amp2_send, amp1_recv) = sync_channel(5);

        // let sender = amp1_send.clone();
        let amp1 = thread::spawn(move || {
            let rec1 = amp1_recv.recv();
            // let rec2 = amp1_recv.recv();

            println!("{:?}",rec1);
            // println!("{:?}",rec2);
        });
        // let sender = amp2_send.clone();
        let amp2 = thread::spawn(move || { println!("{}",amp2_recv.recv().unwrap()); });

        // amp1_send.send(1).expect("Couldn't send phase setting!");
        amp2_send.send(2).expect("Couldn't send init val!");
        amp1_send.send(3).expect("Couldn't send phase setting!");


        amp1.join().unwrap();
        amp2.join().unwrap();
    }
    #[test]
    fn test_feedback_loop() {
        let mut vec = vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                           27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5];
        println!("{}", threaded_phase_setter(vec, vec![9, 8, 7, 6, 5]));
    }
}
