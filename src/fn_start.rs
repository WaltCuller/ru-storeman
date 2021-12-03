use std::os::raw::c_int;
use std::sync::mpsc::Receiver;
use std::process;

pub fn start(rx: Receiver<c_int>) {
    for r in rx {
        println!("???: {}", r);
        process::exit(r)
    }
    loop {

    }
}