use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod components;

use components::*;

fn step(sys : &mut System, _sigterm : &Arc<AtomicBool>) {
    sys.clk.cycle();
    while sys.cpu.lock().unwrap().tcu != 1 {
        sys.clk.cycle();
    }
}

fn step_over(sys : &mut System, sigterm : &Arc<AtomicBool>) {
    if sys.cpu.lock().unwrap().ir.0 == components::cpu::Instruction::JSR {
        let breakpoint = sys.cpu.lock().unwrap().pc + 3;
        run(sys, sigterm, Some(breakpoint));
    } else {
        step(sys, sigterm);
    }
}

fn run(sys : &mut System, sigterm : &Arc<AtomicBool>, breakpoint : Option<u16>) {
    sigterm.store(false, Ordering::Relaxed);

    loop {
        step(sys, sigterm);

        if sigterm.load(Ordering::Relaxed) {
            break;
        }

        if sys.cpu.lock().unwrap().is_halted() {
            break;
        }

        if let Some(bp) = breakpoint {
            if sys.cpu.lock().unwrap().pc == bp {
                break;
            }
        }
    }
}

fn main() {
    let sigterm = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&sigterm)).unwrap();

    env_logger::init();

    let mut sys = System::new();
    let mut last_command : Option<String> = None;

    loop {
        print!("emu> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        if command.len() == 0 {
            break;
        }

        command = command.trim_end().to_string();
        if command.len() == 0 {
            if let Some(c) = last_command {
                command = c;
            }
        }

        match &*command {
            "run" | "r" => {
                run(&mut sys, &sigterm, None);
                println!();
                sys.show_cpu();
                sys.show_per();
            }
            "step" | "s" => {
                step(&mut sys, &sigterm);
                sys.show_cpu();
                sys.show_per();
            }
            "over" | "so" | "o" => {
                step_over(&mut sys, &sigterm);
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
            "ram" | "mem" | "m" => sys.show_ram(),
            "display" | "dsp" | "d" => sys.show_dsp(),
            "quit" | "q" => {
                return;
            }
            "" => { }
            u => {
                println!("unknown command: '{}'", u);
            }
        }

        last_command = Some(command);
    }
}
