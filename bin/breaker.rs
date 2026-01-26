use std::path::PathBuf;

use clap::Parser;
use cryptopals::functions::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let to_break = match (args.ciphertext, args.file) {
        (Some(c), _) => c,
        (None, Some(f)) => std::fs::read_to_string(f)?,
        (None, None) => return Err("No input provided".into()),
    };
    let broken = match args.break_method {
        BreakMethod::RepeatingKeyXOR => {
            let key = break_repeating_key_xor(&to_break.as_bytes())?;
            let plaintext = decrypt_repeating_key_xor(&to_break.as_bytes(), &key);
            plaintext
        }
    };
    println!("{}", String::from_utf8(broken)?);
    Ok(())
}

#[derive(Parser)]
struct Args {
    break_method: BreakMethod,

    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long)]
    ciphertext: Option<String>,
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum BreakMethod {
    RepeatingKeyXOR,
}
