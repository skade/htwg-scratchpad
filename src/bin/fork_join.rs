extern crate futures;
extern crate tokio_timer;
extern crate tokio_core;
extern crate htwg_scratchpad;

use htwg_scratchpad::*;
use tokio_timer::Timer;
use std::time::Duration;

use tokio_core::reactor::Core;

use futures::future::join_all;

fn main() {
    let timer = Timer::default();

    let sleep = timer.sleep(Duration::from_millis(1500))
                     .inspect("sleep");
    let sleep_shorter = timer.sleep(Duration::from_millis(500))
                     .inspect("short sleep");

    let join = join_all(vec![sleep, sleep_shorter])
               .inspect("join");

    let mut core = Core::new().unwrap();
    let result = core.run(join);

    println!("{:?}", result);
}
