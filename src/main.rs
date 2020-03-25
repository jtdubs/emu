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
            "run" | "r" => {
                term.store(false, Ordering::Relaxed);
                while !term.load(Ordering::Relaxed) {
                    sys.clk.cycle();
                    while sys.cpu.lock().unwrap().tcu != 1 {
                        sys.clk.cycle();
                    }

                    if sys.cpu.lock().unwrap().is_halted() {
                        break;
                    }
                }
                println!();
                sys.show_cpu();
                sys.show_per();
            }
            "step" | "s" => {
                sys.clk.cycle();
                while sys.cpu.lock().unwrap().tcu != 1 {
                    sys.clk.cycle();
                }
                sys.show_cpu();
                sys.show_per();
            }
            "sys" => {
                sys.show_cpu();
                sys.show_per();
            }
            "cpu" => sys.show_cpu(),
            "per" => sys.show_per(),
            "zp" | "z" => sys.show_zp(),
            "stack" | "sta" => sys.show_stack(),
            "display" | "dsp" => sys.show_dsp(),
            "quit" | "q" => {
                return;
            }
            "" => {}
            u => {
                println!("unknown command: '{}'", u);
            }
        }
    }
}
