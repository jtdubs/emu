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
                term.store(false, Ordering::Relaxed);
                while !term.load(Ordering::Relaxed) {
                    while sys.cpu.lock().unwrap().tcu != 1 {
                        sys.clk.cycle();
                    }

                    if sys.cpu.lock().unwrap().is_halted() {
                        break;
                    }
                }
                println!();
                sys.show();
            }
            "step" => {
                sys.clk.cycle();
                while sys.cpu.lock().unwrap().tcu != 1 {
                    sys.clk.cycle();
                }
                println!();
                sys.show();
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
