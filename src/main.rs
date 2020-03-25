use std::io::{self, Write};

mod components;
mod system;

use system::System;

fn main() {
    env_logger::init();

    let mut sys = System::new();
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

        match words.next().unwrap() {
            "run" | "r" => {
                sys.run();
                sys.show_cpu();
                sys.show_per();
            }
            "step" | "s" => {
                sys.step();
                sys.show_cpu();
                sys.show_per();
            }
            "over" | "so" | "o" => {
                sys.step_over();
                sys.show_cpu();
                sys.show_per();
            }
            "out" | "up" | "u" | "finish" | "fin" => {
                sys.step_out();
                sys.show_cpu();
                sys.show_per();
            }
            "sys" => {
                sys.show_cpu();
                sys.show_per();
            }
            "bp" => sys.list_breakpoints(),
            "break" => {
                let addr: u16 = words.next().unwrap().parse().unwrap();
                sys.add_breakpoint(addr);
            }
            "del" => {
                let ix: usize = words.next().unwrap().parse().unwrap();
                sys.remove_breakpoint(ix);
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
            "" => {}
            u => {
                println!("unknown command: '{}'", u);
            }
        }

        last_command = Some(command);
    }
}
