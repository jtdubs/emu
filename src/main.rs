use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod components;

use components::*;

fn main() {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term)).unwrap();

    env_logger::init();

    let mut sys = System::new();

    loop {
        print!("emu> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.len() == 0 {
            break;
        }

        match buffer.trim_end() {
            "run" => {
                println!("Running...");
                term.store(false, Ordering::Relaxed);
                while !term.load(Ordering::Relaxed) {
                    if sys.cpu.lock().unwrap().is_halted() {
                        break;
                    }
                    sys.clk.cycle();
                }
                println!();
                println!("CPU: {:x?}", sys.cpu.lock().unwrap());
                println!("PER: {:x?}", sys.per.lock().unwrap());
                println!("DSP: {:x?}", sys.dsp.lock().unwrap());
                println!("CON: {:x?}", sys.con.lock().unwrap());
            }
            "quit" => {
                return;
            }
            "" => {}
            u => {
                println!("unknown command: '{}'", u);
            }
        }
    }
}
