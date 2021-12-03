use std::os::raw::c_int;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::process;
use std::thread;

use signal_hook::consts::*;
use signal_hook::iterator::Signals;

pub fn notify_channel() -> Receiver<c_int> {
    let (tx, rx) = mpsc::channel();

    let mut signals = Signals::new(&[SIGUSR1, SIGUSR2, SIGTERM]).unwrap();
    let handle = signals.handle();
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            tx.send(sig).unwrap();
            match sig {
                SIGUSR1 => {
                },
                SIGUSR2 => {
                },
                SIGTERM => {
                    println!("{}", "SIGTERM");
                    handle.close();
                    process::exit(1)
                }
                _ => unreachable!(),
            }
        }
    });
    rx
}