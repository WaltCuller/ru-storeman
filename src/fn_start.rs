use std::os::raw::c_int;
use std::process;
use std::process::Stdio;
use std::thread;

use crossbeam_channel::{Receiver, select};
use nix::libc::pid_t;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

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
    thread::spawn(move || {
        let mut child = process::Command::new("go").
            stdout(Stdio::inherit()).
            stderr(Stdio::inherit()).
            stdin(Stdio::null()).
            args(["run", "web.go"]).
            spawn().expect("test?");
        pid = child.id();
        let pgid = u32_to_pgid(pid);
        println!("in pid {}", pid);
        println!("in pgid {}", pgid);
        signal::killpg(Pid::from_raw(u32_to_pgid(pid)), Signal::SIGKILL);
        println!("finish")
    });
    thread::spawn(|| {
        process::Command::new("pwd").
            stdout(Stdio::inherit()).
            stderr(Stdio::inherit()).
            stdin(Stdio::null()).
            output().expect("pwd");
    });

    loop {
        println!("in loop");
        select! {
            recv(rx) -> _ => {
                let pgid = u32_to_pid(pid);
                println!("kill {}", pid);
                println!("pgid {}", pgid);
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