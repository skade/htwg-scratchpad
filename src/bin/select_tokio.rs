extern crate futures;
extern crate tokio_timer;
extern crate tokio_core;
extern crate htwg_scratchpad;

use htwg_scratchpad::*;
use tokio_timer::Timer;
use std::time::Duration;

use futures::future::select_all;
use tokio_core::reactor::Core;

fn main() {
    let timer = Timer::default();

    let sleep = timer.sleep(Duration::from_millis(1000))
                     .inspect("sleep");
    let short_sleep = timer.sleep(Duration::from_millis(500))
                     .inspect("short sleep");
    
    let select = select_all(vec![sleep, short_sleep])
                 .inspect("first select");

    let mut core = Core::new().unwrap();

    let (result, index, remaining_futures) = core.run(select).unwrap();
    
    println!("Future with index {} returned {:?}", index, result);

    let select = select_all(remaining_futures)
                .inspect("second select");

    let (result, index, _) = core.run(select).unwrap();

    println!("Future with index {} returned {:?}", index, result);
}
