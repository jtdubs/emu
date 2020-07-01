use std::io::{self, Write};
use std::env;

mod components;
mod debugger;
mod breadboard_system;
mod cpu_test_system;
mod system;

use debugger::Debugger;
use breadboard_system::BreadboardSystem;
use cpu_test_system::CPUTestSystem;
use system::System;

fn main() {
    env_logger::init();

    match env::args().nth(1).unwrap_or("breadboard".to_string()).as_str() {
        "cpu_test" => {
            run(Debugger::new(CPUTestSystem::new("6502_functional_test.bin", 0x400)))
        }
        "breadboard" => {
            let mut d = Debugger::new(BreadboardSystem::new("rom.bin"));
            d.read_symbols("rom.sym");
            run(d);
         }
        _ => { panic!("invalid board"); }
    };
}

fn run<SystemType: System>(mut dbg: Debugger<SystemType>) {        
    let mut last_command: Option<String> = None;

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

        let mut words = command.split_ascii_whitespace();

        match words.next().unwrap_or("") {
            "run" | "r" => {
                dbg.run();
                dbg.show_cpu();
                dbg.show_per();
            }
            "bench" => {
                dbg.bench();
            }
            "headless" | "head" | "rh" => {
                dbg.run_headless();
                dbg.show_cpu();
                dbg.show_per();
            }
            "step" | "s" => {
                dbg.step();
                dbg.show_cpu();
                dbg.show_per();
            }
            "over" | "so" | "o" => {
                dbg.step_over();
                dbg.show_cpu();
                dbg.show_per();
            }
            "out" | "up" | "u" | "finish" | "fin" => {
                dbg.step_out();
                dbg.show_cpu();
                dbg.show_per();
            }
            "sys" => {
                dbg.show_cpu();
                dbg.show_per();
            }
            "bp" => dbg.list_breakpoints(),
            "break" | "br" | "b" => dbg.add_breakpoint(words.next().unwrap()),
            "del" => dbg.remove_breakpoint(words.next().unwrap().parse().unwrap()),
            "cpu" => dbg.show_cpu(),
            "per" => dbg.show_per(),
            "zp" | "z" => dbg.show_zp(),
            "stack" | "sta" => dbg.show_stack(),
            "ram" | "mem" | "m" => dbg.show_ram(),
            "display" | "dsp" | "d" => dbg.show_dsp(),
            "quit" | "q" | "exit" => {
                return;
            }
            "" => {}
            u => {
                println!("unknown command: '{}'", u);
            }
        }

        last_command = Some(command);
    }
}
