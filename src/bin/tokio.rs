extern crate futures;
extern crate tokio_timer;
extern crate tokio_core;
extern crate htwg_scratchpad;

use htwg_scratchpad::*;

use tokio_timer::Timer;
use std::time::Duration;

use tokio_core::reactor::Core;

fn main() {
    let timer = Timer::default();

    let sleep = timer.sleep(Duration::from_millis(1000))
                     .inspect("sleep");

    let mut core = Core::new().unwrap();

    let task = core.run(sleep);

    println!("{:?}", task);
}
