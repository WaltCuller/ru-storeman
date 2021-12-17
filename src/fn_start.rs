use std::os::raw::c_int;
use std::process::{self, Child};
use std::process::Stdio;
use std::thread;

use crossbeam_channel::{Receiver, select};
use nix::libc::{pid_t, signal};
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use signal_hook::consts::*;

pub fn start(rx: Receiver<c_int>) {
    let cmd = Procfile {
        procfile: "".to_string(),
        port: 0,
        base_dir: "".to_string(),
        base_port: 0,
        exit_on_error: true,
    };
    let proc_info = ProcInfo {
        name: "".to_string(),
        cmd_line: "".to_string(),
        port: 0,
        set_port: false,
        color_index: 0,
        stopped_by_supervisor: false,
    };

    start_procs(rx, proc_info, cmd.exit_on_error);
}

struct ProcInfo {
    name: String,
    cmd_line: String,
    port: u64,
    set_port: bool,
    color_index: i64,
    stopped_by_supervisor: bool,

}

struct Procfile {
    procfile: String,
    port: u64,
    base_dir: String,
    base_port: u64,
    exit_on_error: bool,
}

fn start_procs(rx: Receiver<c_int>, info: ProcInfo, exit_on_error: bool) {
    let mut pid: u32 = 0;

    thread::spawn(|| {
        process::Command::new("/bin/sh").
            arg("-c").arg("echo hello").
            stdout(Stdio::inherit()).
            stderr(Stdio::inherit()).
            stdin(Stdio::null()).spawn().
            expect("pwd");
    });

    let ot = thread::spawn(move || -> Child {
        let mut child = process::Command::new("/bin/sh").
            arg("-c").arg("go run web.go").
            stdout(Stdio::inherit()).
            stderr(Stdio::inherit()).
            stdin(Stdio::null()).spawn().
            expect("test?");
        return child;
    });

    let mut res = ot.join().expect("");

    loop {
        println!("in loop");
        select! {
            recv(rx) -> c => {
                println!("signal: {}", c.unwrap());
                println!("pid {}", res.id());
                res.kill().expect("????");
                println!("finish")
            },
        }
    }
}

fn u32_to_pgid(pid: u32) -> pid_t {
    let pgid: i32 = pid as i32;
    -1 * pgid
}

fn u32_to_pid(pid: u32) -> pid_t {
    pid as i32
}