use std::io;
use std::io::Read;

use regex::Regex;
use clap::Parser;

use capstone::prelude::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the target function, please specify in "<...>"
   #[arg(short, long)]
   target_function: String,
   /// dump shellcode or C struct 
   #[arg(short, long, default_value_t = false)]
   structmode: bool,
}

/// read from input in a streaming fashion
pub fn reader<R: Read>(stdin: R, target: &str, 
    structmode: bool) -> io::Result<usize> {

    let mut base_addr = 0;

    let mut collect = false;
    let re = Regex::new(r"<.*>").unwrap();

    let mut line = Vec::new();
    let mut shellcode = "".to_string();
    let mut opcodes: Vec<(String, usize)> = Vec::new();

    // Read Trait allows reading bytes from a source, has method `bytes`
    for i in stdin.bytes() {

        let done = match i {
            Ok(c) => {
                match c {
                    0xa => {
                        true
                    },
                    _ => {
                        line.push(c);
                        false
                    },
                }
            },
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            },
        };

        // process the line
        if done {
            // this means we have a function
            if line.len() != 0 && line[0] != 0x20 && line.contains(&0x3c){

                collect = false;

                let line = String::from_utf8_lossy(&line).to_string();
                let cap = re.captures(&line).unwrap();
                let text = cap
                    .get(0)
                    .map_or("", |m| m.as_str());
                if text == target {
                    collect = true;
                }
            }

            if collect {
                let line = String::from_utf8_lossy(&line).to_string();
                let split: Vec<String> = line.split("\t")
                    .map(|x| x.to_string()).collect();
                if split.len() > 1 {
                    // first opcode byte must also have a \x
                    match shellcode.len() {
                        0 => {
                            base_addr = u64::from_str_radix(&split[0]
                                .trim()
                                .replace(":", ""), 16).unwrap();

                            shellcode.push_str("\\x")
                        },
                        _ => shellcode.push_str(" "),
                    }
                    let s = split[1].trim_end().replace(" ", "\\x");
                    shellcode.push_str(&s);
                    if structmode {
                        opcodes.push(("\\x".to_owned() + &s, s.matches("\\x").count() + 1))
                    }
                }
            }

            line.clear();
        }

    } // for input bytes

    if structmode {

        let cs = Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode64)
            .syntax(arch::x86::ArchSyntax::Intel)
            .detail(true)
            .build()
            .expect("Failed to create Capstone object");

        for item in &opcodes {

            let mut s_instr = String::new();

            let code: Vec<u8> = item.0
                .split("\\x")
                .filter(|x| !x.is_empty())
                .map(|x| 
                    match u8::from_str_radix(x,16) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("error: {e}");
                            std::process::exit(1);
                        }
                    }
                )
                .collect();

            let insns = cs
                .disasm_all(&code, base_addr)
                .expect("error disassembling");
            
            for i in insns.as_ref() {
                s_instr.push_str( &format!("{}", i) );
            }

            println!("{{ {:#08x}, \"{}\", {} }}, // {}", 
                base_addr, item.0, item.1, s_instr);
            base_addr += item.1 as u64;
        }
        return Ok(opcodes.len());
    } else {
        print!("{}\n", shellcode.replace(" ", "\\x"));
    }

    Ok(shellcode.len())
}

fn main() {

    let stdin = io::stdin();
    let args = Args::parse();

    let res = reader(stdin.lock(),
        &args.target_function, args.structmode);

    match res {
        Ok(s) => println!("produced {} bytes of output", s),
        Err(e) => eprintln!("error: {e}"),
    };

}   
