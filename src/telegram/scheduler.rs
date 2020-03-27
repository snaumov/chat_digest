// pub struct Scheduler;

// impl Scheduler {

// }

use clokwerk::{Scheduler as ClokwerkScheduler, TimeUnits};
use std::time::Duration;
use clokwerk::Interval::*;
use super::digest::{Digest, DigestError};
use std::thread;
use std::marker::Send;

// #[derive(Send)]
// struct Scheduler<'a> {
//     digest: Digest<'a>,
//     scheduler: ClokwerkScheduler,
// }

// impl<'a> Scheduler<'a> {
//     pub fn new(digest: Digest) -> Scheduler {

//         let mut scheduler = ClokwerkScheduler::new();

//         Scheduler {
//             digest,
//             scheduler,
//         }
//     }

pub fn run<'a>(digest: Digest) {
    let mut scheduler = ClokwerkScheduler::new();

    scheduler.every(5.seconds()).run(move || { 
        digest.build_digest();
        // Ok(())
    });

    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    thread_handle.stop();
}
// }