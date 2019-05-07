extern crate inputbot;
extern crate rand;

use rand::Rng;

use inputbot::*;
use std::{thread::sleep, thread::spawn, time::Duration};
use MouseButton::*;

fn main() {
    loop {
        let num = rand::thread_rng().gen_range(-50, 50);
        MouseCursor.move_rel(10, num);
        sleep(Duration::from_millis(1500));
    }
}
