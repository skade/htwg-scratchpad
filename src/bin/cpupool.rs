extern crate futures;
extern crate tokio_timer;
extern crate futures_cpupool;
extern crate htwg_scratchpad;

use htwg_scratchpad::*;
use tokio_timer::Timer;
use std::time::Duration;

use futures::{Future};
use futures_cpupool::CpuPool;

fn main() {
    let timer = Timer::default();

    let sleep = timer.sleep(Duration::from_millis(1000))
                     .inspect("sleep");

    let cpu_pool = CpuPool::new(4);

    let task = cpu_pool.spawn(sleep);

    println!("{:?}", task.wait());
}
