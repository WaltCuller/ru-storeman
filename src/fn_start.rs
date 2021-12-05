use std::os::raw::c_int;
use std::{process, time};
use crossbeam_channel::{select, Receiver, after};

pub fn start(rx: Receiver<c_int>) {

    let timeout = after(time::Duration::from_secs(5));

    loop {
        println!("in loop");
        select! {
            recv(rx) -> _ => {
                println!("???");
                process::exit(10)
            },
            recv(timeout) -> _ => {
                println!("break");
                process::exit(11)
            }
        }
    }
}