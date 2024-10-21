use std::ffi;

use libc::{c_void, O_CREAT, O_TRUNC, O_WRONLY, S_IRWXU};

/*
    To keep it simple, every system call has an assertion that the operation results in a non-failure.
*/

fn spawn_fork () -> i32 {
    let mut pid = -1;
    
    unsafe {
        pid = libc::fork();
        assert!(pid >= 0, "It wasn't possible to spawn a child!");
    }

    return pid;
}

fn close_file (fd: i32) {
    unsafe {
        let s = libc::close(fd);
        assert!(s >= 0, "Failed to close file-descriptor: {fd}");
    }
}

fn wait_a_child () -> i32{ // The wait() system call suspends execution of the calling thread until one of its children terminates.
    let mut w_status = -1;

    unsafe {
        w_status = libc::wait(std::ptr::null_mut());
        assert!(w_status >= 0);
    }
    
    return w_status;
}

fn write_to_file (fd: i32, str: &str) -> isize {
    let msg = ffi::CString::new(str).unwrap();

    let b_count = msg.to_bytes().len();
    let buffer = msg.as_ptr() as *const c_void;

    let mut w_result: isize = -1;

    unsafe {
        w_result = libc::write(fd, buffer, b_count);
        assert!(w_result >= 0, "Error writing {fd}. Error: {w_result}")
    }

    return w_result;
}

fn open_file(path: &str, flags: i32, mode: u32) ->  i32 {
    let mut fd = -1;
    let c_string = ffi::CString::new(path).unwrap();

    unsafe {
        fd = libc::open(c_string.as_ptr(), flags, mode);
        assert!(fd >= 0, "It wasn't possible to open the file.");
    }

    return fd;
}

fn main () {
    let fd = open_file("./test_ostep_ffi.txt", O_CREAT|O_WRONLY|O_TRUNC, S_IRWXU as u32);
    let p_pid = std::process::id();
    let mut x = 100;

    println!("Parent pid: {p_pid}");

    println!("File descriptor pre-fork: {fd}");

    x += 10;

    println!("Still the parent"); // executed once

    let c_pid = spawn_fork();

    let common_msg = format!("Common instructions. Child ({c_pid}) and Parent ({p_pid}) are both writing.");

    println!("{common_msg}"); // this is executed "twice" since the child is spawned (forked) here and now it will execute every instruction from here

    x += 10; // the instructions previous to the fork AREN'T executed by the child. Here it has 110 (+10) cause the parent's memory is copied when the child is spawned thus it holds the same value in memory (though it isn't shared anymore)

    println!("File descriptor post fork is: {fd}");

    write_to_file(fd, &common_msg);

    match c_pid {
        0 => {
            x += 3;

            let str = format!("It's the child ({c_pid}). X is: {x}");

            println!("{str}"); // 123
            write_to_file(fd, &str);
        },
        _ => {
            wait_a_child();
            x += 2; // 122
            let str = format!("It's the parent ({c_pid}). X is: {x}");

            println!("{str}");
            write_to_file(fd, &str);
        }
    }

    close_file(fd);
}