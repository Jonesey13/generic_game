use bitflags;
use std::env;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Write, Stderr, Result, stderr};
use std::collections::HashMap;
pub mod clock;
mod clock_writer;
use std::sync::Mutex;

pub static mut DEBUGGER: Debugger = Debugger {
    flags: DEFAULTDEBUG,
};

lazy_static! {
    static ref CLOCKWRITER: Mutex<clock_writer::ClockWriter> = Mutex::new(clock_writer::ClockWriter::default());
}

pub struct Debugger {
    flags: DebugFlags,
}

const LOGFILEPATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), r"\log.txt");

bitflags! {
    flags DebugFlags: u32 {
        const WRITETOCONSOLE  = 0b00000001,
        const WRITETOFILE     = 0b00000010,
        const DEBUGCOLLISION  = 0b00000100,
        const DEBUGRENDERING  = 0b00001000,
        const DEBUGINPUT      = 0b00010000,
        const DEBUGGAME       = 0b00100000,
        const DEBUGCLOCKS     = 0b01000000,
        const DEBUGALL        = 0b11111111,
        const DEFAULTDEBUG    = WRITETOCONSOLE.bits | DEBUGCLOCKS.bits,
    }
}

pub fn debug(mes: &str) {
    log_to_file(mes);
    log_to_console(mes);
}

pub fn log_to_file(mes: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(WRITETOFILE) {
            let mut file = match OpenOptions::new().write(true).append(true).create(true).open(LOGFILEPATH) {
                Ok(f) => f,
                Err(_) => panic!("Could not open log file at {}", LOGFILEPATH)
            };
            match writeln!(file, "{}", mes) {
                Ok(_) => (),
                Err(_) => panic!("Could not write to log file!")
            }
        }
    }
}

pub fn log_to_file_bytes(mes: &[u8]) {
    unsafe {
        if DEBUGGER.flags.intersects(WRITETOFILE) {
            let mut file = match OpenOptions::new().write(true).append(true).create(true).open(LOGFILEPATH) {
                Ok(f) => f,
                Err(_) => panic!("Could not open log file at {}", LOGFILEPATH)
            };
            match file.write(mes) {
                Ok(_) => (),
                Err(_) => panic!("Could not write to log file!")
            }
        }
    }
}

fn log_to_console(mes: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(WRITETOCONSOLE) {
            println!("{}", mes);
        }
    }
}

pub fn debug_clock(mes: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGCLOCKS) {
            debug(mes);
        }
    }
}

pub fn debug_clock_start(clock_name: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGCLOCKS) {
            CLOCKWRITER.lock().unwrap().start_clock(clock_name.to_string());
        }
    }
}

pub fn debug_clock_start_main() {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGCLOCKS) {
            CLOCKWRITER.lock().unwrap().start();
        }
    }
}

pub fn debug_clock_stop(clock_name: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGCLOCKS) {
            CLOCKWRITER.lock().unwrap().stop_clock(clock_name.to_string());
        }
    }
}

pub fn debug_clock_stop_main() {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGCLOCKS) {
            CLOCKWRITER.lock().unwrap().stop();
        }
    }
}

pub fn debug_inp(mes: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGINPUT) {
            debug(mes);
        }
    }
}

pub fn debug_game(mes: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGGAME) {
            debug(mes);
        }
    }
}

pub fn debug_rend(mes: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGRENDERING) {
            debug(mes);
        }
    }
}

pub fn debug_coll(mes: &str) {
    unsafe {
        if DEBUGGER.flags.intersects(DEBUGCOLLISION) {
            debug(mes);
        }
    }
}

pub fn set_flags(flags: DebugFlags) {
    unsafe{
        DEBUGGER.flags = flags;
    }
}

pub struct ErrorWriter {
    pub stderr: Stderr
}

impl Write for ErrorWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        log_to_file_bytes(buf);
        self.stderr.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.stderr.flush()
    }
}

impl ErrorWriter {
    pub fn new() -> ErrorWriter {
        ErrorWriter {
            stderr: stderr()
        }
    }
}
