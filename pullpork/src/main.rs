use std::fs;
use std::env;
use std::path::Path;

#[derive(Debug)]
enum PullError {
    RangeError
}

fn pull(path: &Path, start: usize, end: usize) -> Result<Vec<u8>, PullError> {

    let content = match fs::read(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error {e}");
            std::process::exit(1);
        }
    };

    if start >= content.len() || end > content.len() || start == end {
        return Err(PullError::RangeError);
    }

    Ok(content[start..end].to_vec())
}

fn usage() {
    println!("usage: ./pullpork [start] [end]");
    println!("start and end as hex without prefix");
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        usage();
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);

    let start = match usize::from_str_radix(&args[2], 16) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[!] start position error: {e}");
            std::process::exit(1);
        }
    };


    let end = match usize::from_str_radix(&args[3], 16) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[!] end position error: {e}");
            std::process::exit(1);
        }
    };

    if path.exists() {
        match pull(path, start, end) {
            Ok(v) => {
                let filename = format!("dump_{}_{}.dat", start, end);
                println!("[+] pull ok, writing output to: {}", filename);
                match fs::write(&filename, v) {
                    Ok(()) => println!("[+] wrote output"),
                    Err(e) => {
                        eprintln!("[!] error writing output: {e}");
                        std::process::exit(1);
                    }
                } // match inner
            },
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1);
            }
        } // match outer
    } else {
        println!("[!] path [{}] does not exist", (args[1]));
    }

}
