extern crate futures;
extern crate tokio_timer;

extern crate futures_cpupool;

extern crate htwg_scratchpad;

use htwg_scratchpad::*;
use tokio_timer::Timer;
use std::time::Duration;

use futures::{Future};
use futures::future::select_all;
use futures_cpupool::CpuPool;

fn main() {
    let timer = Timer::default();

    let sleep = timer.sleep(Duration::from_millis(1000))
                     .inspect("sleep");
    let short_sleep = timer.sleep(Duration::from_millis(900))
                     .inspect("short sleep");
    
    let select = select_all(vec![sleep, short_sleep]);

    let cpu_pool = CpuPool::new(4);

    let task = cpu_pool.spawn(select);
    
    let (result, index, futures) = task.wait().unwrap();

    println!("Future with index {} returned {:?}", index, result);

    let select = select_all(futures);

    let task = cpu_pool.spawn(select);

    let (result, index, _) = task.wait().unwrap();

    println!("Future with index {} returned {:?}", index, result);

}
