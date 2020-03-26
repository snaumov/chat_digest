// pub struct Scheduler;

// impl Scheduler {

// }

use clokwerk::{Scheduler};
use std::time::Duration;
use clokwerk::Interval::*;
use super::digest;

pub fn run() {
    let mut scheduler = Scheduler::new();

    scheduler.every(1.minutes()).run(|| println!("Hi"));

    scheduler.watch_thread(Duration::from_millis(100));
}