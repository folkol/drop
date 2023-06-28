use std::{io, io::prelude::*};
use std::collections::VecDeque;
use std::io::BufWriter;

use clap::Parser;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
struct Args {
    #[clap(default_value_t = false, short, long)]
    tail: bool,
    #[clap(default_value_t = 10)]
    n: usize,
}

pub fn main() -> Result<()> {
    let args = Args::parse();
    let mut out = BufWriter::new(io::stdout().lock());

    if args.tail {
        let mut queue = VecDeque::<String>::new();
        let lines = io::stdin().lock().lines();
        for line in lines.take(args.n) {
            queue.push_back(line?)
        }
        for line in io::stdin().lines() {
            queue.push_back(line?);
            let output = queue.pop_front().expect("Expected line in queue");
            writeln!(out, "{}", output)?
        }
    } else {
        for line in io::stdin().lines().skip(args.n) {
            writeln!(out, "{}", line?)?
        }
    };
    out.flush()?;
    Ok(())
}
