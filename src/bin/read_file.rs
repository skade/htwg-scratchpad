extern crate futures;
extern crate futures_cpupool;

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use futures_cpupool::CpuPool;
use futures::future::{join_all, lazy};

use futures::Future;

fn main() {
    let cpu_pool = Arc::new(CpuPool::new(2));

    let open_file = lazy(|| {
        fs::File::open("lines")
            .map(|f| {
                let buf_reader = BufReader::new(f);
                Arc::new(Mutex::new(buf_reader))
            })
    });
    
    let shared_pool = cpu_pool.clone();

    let read_file = open_file.and_then(move |file_handle| {
        let mut tasks = vec![];

        for _ in 0..2 {
            let handle = file_handle.clone();

            let task = shared_pool.spawn_fn(move || {
                let mut string = String::new();
                let read_line = handle.lock().unwrap().read_line(&mut string);

                read_line.and_then(|_| {
                        println!("{}", string.trim());
                        Ok(())
                    })
            });

            tasks.push(task)
        }

        join_all(tasks)
    });

    cpu_pool.spawn(read_file).wait();
}
