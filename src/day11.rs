use std::collections::HashSet;
use std::sync::mpsc::{sync_channel, RecvError, SendError, TryRecvError, TrySendError};
use std::{fs, thread};
use crate::day9::computer_ver5;

const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';

#[derive(Debug)]
struct Canvas {
    prev_direction: char,
    location: (usize, usize),
    pixels: Vec<Vec<char>>,
    painted_areas: HashSet<(usize, usize)>,
}

impl Canvas {
    fn new() -> Self {
        let mut pixels = vec![vec!['.'; 500]; 500];
        let mut painted_areas = HashSet::new();
        let location = (250, 250);
        pixels[location.0][location.1] = '#';
        Canvas { prev_direction: '^', location, pixels, painted_areas }
    }

    fn paint_canvas(&mut self, color: usize) {
        let color = match color {
            0 => { '.' }
            1 => { '#' }
            _ => {
                eprintln!("unmatched color");
                return;
            }
        };
        self.pixels[self.location.0][self.location.1] = color;
        if color == '#' { self.painted_areas.insert((self.location.0, self.location.1)); }
    }

    fn move_arrow(&mut self, rotation: usize) {
        match self.prev_direction {
            UP => {
                match rotation {
                    0 => { self.prev_direction = LEFT }
                    1 => { self.prev_direction = RIGHT }
                    _ => {
                        eprintln!("unmatched rotation");
                        return;
                    }
                }
            }
            DOWN => {
                match rotation {
                    0 => { self.prev_direction = RIGHT }
                    1 => { self.prev_direction = LEFT }
                    _ => {
                        eprintln!("unmatched rotation");
                        return;
                    }
                }
            }
            LEFT => {
                match rotation {
                    0 => { self.prev_direction = DOWN }
                    1 => { self.prev_direction = UP }
                    _ => {
                        eprintln!("unmatched rotation");
                        return;
                    }
                }
            }
            RIGHT => {
                match rotation {
                    0 => { self.prev_direction = UP }
                    1 => { self.prev_direction = DOWN }
                    _ => {
                        eprintln!("unmatched rotation");
                        return;
                    }
                }
            }
            _ => {
                eprintln!("unmatched direction");
                return;
            }
        }
        self.move_location(self.prev_direction);
    }

    fn move_location(&mut self, move_direction: char) {
        match move_direction {
            UP => { self.location = (self.location.0, self.location.1 - 1); }
            DOWN => { self.location = (self.location.0, self.location.1 + 1); }
            LEFT => { self.location = (self.location.0 - 1, self.location.1); }
            RIGHT => { self.location = (self.location.0 + 1, self.location.1); }
            _ => { eprintln!("unmatched direction"); }
        }
    }

    fn save_canvas(&self) {
        let mut res_str = String::from("");
        for row in self.pixels.clone() {
            let append:String = row.iter().collect();
            res_str += append.as_str();
            res_str.push('\n');
        }
        fs::write("./outputs/day11", res_str).unwrap();
    }
}
pub fn run_computer(vec: Vec<i64>) {
    let (sender1, receiver2) = sync_channel(10);
    let (sender2, receiver1) = sync_channel(10);

    let handle = thread::spawn(move || { computer_ver5(vec, sender1, receiver1) });

    let mut canvas = Canvas::new();
    let painter = thread::spawn(move || {
        loop {
            let (x, y) = canvas.location;
            // println!("{}", canvas.pixels[x][y]);
            let cur_col = match canvas.pixels[x][y] {
                '.' => { 0 }
                '#' => { 1 }
                _ => { break; }
            };
            match sender2.try_send(cur_col) {
                Ok(_) => {}
                Err(TrySendError::Disconnected(..)) => { break; }
                Err(e) => { eprintln!("{e}") }
            };
            let color = match receiver2.recv() {
                Ok(color) => { color as usize }
                // Err(TryRecvError::Disconnected) => {eprintln!("disconnected"); break;},
                Err(e) => {
                    eprintln!("{e:?}");
                    break;
                }
            };

            let rotation = match receiver2.recv() {
                Ok(rotation) => { rotation as usize }
                // Err(TryRecvError::Disconnected) => {eprintln!("disconnected");break;},
                Err(e) => {
                    eprintln!("{e:?}");
                    break;
                }
            };


            canvas.paint_canvas(color);
            canvas.move_arrow(rotation);
        }
        canvas
    });


    let res = handle.join();
    let output = painter.join().unwrap();
    output.save_canvas();
    println!("{:?}", output.painted_areas.len());
}
// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_robot_eye_message() {}
// }