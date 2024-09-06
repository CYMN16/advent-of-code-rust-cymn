use std::collections::HashSet;
use std::sync::mpsc::{sync_channel, Receiver, RecvError, SendError, SyncSender, TryRecvError, TrySendError};
use std::{fs, thread};
use std::io::stdin;
use std::num::ParseIntError;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ruscii::app::{App, State};
use ruscii::terminal::{Window};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};


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
struct Computer {
    memory: Vec<i64>,
    memory_start_index: usize,
    instruction_pointer: usize,
    relative_base: i64,
    sender: SyncSender<i64>,
    receiver: Receiver<i64>,
    result: i64,
}

impl Computer {
    fn new(program: Vec<i64>, memory_size: usize, sender: SyncSender<i64>, receiver: Receiver<i64>) -> Self {
        let mut memory = program.clone();
        let mut empty_mem = Vec::new();
        empty_mem.resize(memory_size, 0);
        memory.append(&mut empty_mem);

        Self { memory, memory_start_index: program.len(), instruction_pointer: 0, relative_base: 0, sender, receiver, result: -1 }
    }

    fn insert_at_mem(&mut self, index: usize, val: i64) {
        self.memory[index] = val;
    }

    fn run(&mut self) {
        // let mut vec = self.memory.clone();
        let instruction_pointer = self.instruction_pointer;
        loop {
            let (opcode, param_modes) = parse_instruction(self.memory[self.instruction_pointer]);

            match opcode {
                Opcode::Sum => {
                    let param1 = get_param(&self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    let param2 = get_param(&self.memory, self.instruction_pointer + 2, self.relative_base, param_modes[1]);
                    let param3 = get_mut_param(&mut self.memory, self.instruction_pointer + 3, self.relative_base, param_modes[2]);
                    *param3 = param1 + param2;
                    self.instruction_pointer += 4;
                }
                Opcode::Multiply => {
                    let param1 = get_param(&self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    let param2 = get_param(&self.memory, self.instruction_pointer + 2, self.relative_base, param_modes[1]);
                    let param3 = get_mut_param(&mut self.memory, self.instruction_pointer + 3, self.relative_base, param_modes[2]);
                    *param3 = param1 * param2;
                    self.instruction_pointer += 4;
                }
                Opcode::Input => {
                    let param1 = get_mut_param(&mut self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    // println!("Input requested!");

                    *param1 = self.receiver.recv().expect("Did not receive any input!");
                    // println!("received input: {param1:?}");
                    // println!("Received {param1} input!");
                    self.instruction_pointer += 2;
                }
                Opcode::Output => {
                    let param1 = get_param(&self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    match self.sender.try_send(param1) {
                        Ok(_) => {}
                        Err(TrySendError::Disconnected(_)) => { self.result = param1 }
                        Err(e) => { eprintln!("Received {e} error!") }
                    };
                    self.instruction_pointer += 2;
                }
                Opcode::JumpIfTrue => {
                    let param1 = get_param(&self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    let param2 = get_param(&self.memory, self.instruction_pointer + 2, self.relative_base, param_modes[1]);
                    if param1 != 0 {
                        self.instruction_pointer = param2 as usize
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Opcode::JumpIfFalse => {
                    let param1 = get_param(&self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    let param2 = get_param(&self.memory, self.instruction_pointer + 2, self.relative_base, param_modes[1]);
                    if param1 == 0 {
                        self.instruction_pointer = param2 as usize
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Opcode::LessThan => {
                    let param1 = get_param(&self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    let param2 = get_param(&self.memory, self.instruction_pointer + 2, self.relative_base, param_modes[1]);
                    let param3 = get_mut_param(&mut self.memory, self.instruction_pointer + 3, self.relative_base, param_modes[2]);
                    if param1 < param2 {
                        *param3 = 1;
                    } else {
                        *param3 = 0;
                    }
                    self.instruction_pointer += 4;
                }

                Opcode::Equals => {
                    let param1 = get_param(&self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    let param2 = get_param(&self.memory, self.instruction_pointer + 2, self.relative_base, param_modes[1]);
                    let param3 = get_mut_param(&mut self.memory, self.instruction_pointer + 3, self.relative_base, param_modes[2]);

                    if param1 == param2 {
                        *param3 = 1;
                    } else {
                        *param3 = 0;
                    }
                    self.instruction_pointer += 4;
                }
                Opcode::RelativeBaseOffset => {
                    let param1 = get_param(&self.memory, self.instruction_pointer + 1, self.relative_base, param_modes[0]);
                    self.relative_base += param1;
                    self.instruction_pointer += 2;
                }
                Opcode::Stop => {
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
struct GameState {
    pixels: Vec<Vec<char>>,
    score: i64,
    current_input: i64,
    ball_position: Vec2,
    paddle_position: Vec2,
}

impl GameState {
    fn new() -> Self {
        let mut pixels = vec![vec!['.'; 40]; 28];
        Self { pixels, score: 0, current_input: 0, ball_position: Vec2::xy(0,0), paddle_position: Vec2::xy(0,0) }
    }

    fn get_pixels(&self) -> Vec<Vec<char>> {
        self.pixels.clone()
    }

    fn draw_shape(&mut self, tile: usize, x: usize, y: usize) {
        let color = match tile {
            0 => { ' ' } //empty
            1 => { '#' } //wall
            2 => { '=' } //block
            3 => { '-' } //horizontal paddle
            4 => { 'o' } //ball
            _ => {
                eprintln!("unmatched tile {tile:?}");
                return;
            }
        };

        match color {
            'o' => {
                self.ball_position = Vec2::xy(y,x)
            }
            '-' => { self.paddle_position = Vec2::xy(y,x) }
            _ => {}
        }

        self.pixels[x][y] = color;
    }

    fn get_score(&self) -> i64 {
        self.score
    }
    fn update_score(&mut self, score: i64) {
        if score > self.score {
            self.score = score
        }
    }

    fn get_current_input(&self) -> i64 {
        self.current_input
    }

    fn set_current_input(&mut self, direction: i64) {
        self.current_input = direction
    }

    fn calculate_next_move(&mut self) -> i32 {
        // let dx = self.ball_position.x - self.prev_ball_position.x;
        //
        // let direction = match dx {
        //     -1 => {-1},
        //     0 => {0},
        //     1 => {1},
        //     _ => {0},
        // };
        // if self.paddle_position.x != self.ball_position.x {
        //     0
        // } else {
        //     1
        // }
        if self.ball_position.x < self.paddle_position.x {
            -1
        } else if self.ball_position.x > self.paddle_position.x {
            1
        } else {
            0
        }
    }
}


pub fn run_computer_with_ruscii(mut vec: Vec<i64>) {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();
    let mut game_state = Arc::new(Mutex::new(GameState::new()));

    let (sender1, receiver2) = sync_channel(100);
    let (sender2, receiver1) = sync_channel(100);

    let mut computer = Computer::new(vec, 16000, sender1, receiver1);

    computer.insert_at_mem(0, 2);


    let handle = thread::spawn(move || { computer.run() });

    let joystick_game_state = Arc::clone(&game_state);
    let joystick = thread::spawn(move || {
        let mut first = true;
        let mut second = true;
        loop {
            // let message = joystick_game_state.lock().unwrap().get_current_input();
            let mut message = joystick_game_state.lock().unwrap().calculate_next_move() as i64;

            match sender2.send(message) {
                Ok(tile) => { tile }
                Err(e) => {
                    eprintln!("4{e:?}");
                    break;
                }
            };
            // joystick_game_state.lock().unwrap().set_current_input(0);

            thread::sleep(Duration::from_millis(5));
        }
    });
    let drawer_game_state = Arc::clone(&game_state);
    let drawer = thread::spawn(move || {
        loop {
            let x = match receiver2.recv() {
                Ok(x) => { x }
                Err(e) => {
                    eprintln!("1{e:?}");
                    break;
                }
            };
            let y = match receiver2.recv() {
                Ok(y) => { y }
                Err(e) => {
                    eprintln!("2{e:?}");
                    break;
                }
            };
            let tile = match receiver2.recv() {
                Ok(tile) => { tile }
                Err(e) => {
                    eprintln!("3{e:?}");
                    break;
                }
            };
            if x == -1 && y == 0 {
                drawer_game_state.lock().unwrap().update_score(tile);
            } else {
                drawer_game_state.lock().unwrap().draw_shape(tile as usize, y as usize, x as usize);
            }
        };
    });


    // let res = handle.join();

    let main_game_state = Arc::clone(&game_state);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::Left) => game_state.lock().unwrap().set_current_input(-1),
                KeyEvent::Pressed(Key::Down) => game_state.lock().unwrap().set_current_input(0),
                KeyEvent::Pressed(Key::Right) => game_state.lock().unwrap().set_current_input(1),
                _ => (),
            }
        }


        fps_counter.update();
        let mut pencil = Pencil::new(window.canvas_mut());
        // pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));
        for (index, row) in game_state.lock().unwrap().get_pixels().clone().iter().enumerate() {
            let str: String = row.iter().collect();
            pencil.draw_text(str.as_str(), Vec2::xy(0, index));
            // res_str += append.as_str();
            // res_str.push('\n');
        }
        pencil.draw_text(&format!("Max Score: {}", game_state.lock().unwrap().get_score()), Vec2::xy(0, 0));
        pencil.draw_text(&format!("Next direction: {}", game_state.lock().unwrap().get_current_input()), Vec2::xy(20, 0));
    });


}
