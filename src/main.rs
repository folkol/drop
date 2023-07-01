use std::{io, io::prelude::*};
use std::collections::VecDeque;
use std::io::{BufWriter, StdoutLock};

use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
struct Args {
    #[clap(default_value_t = false, short, long)]
    debug: bool,
    #[clap(default_value_t = false, short, long)]
    tail: bool,
    #[clap(default_value_t = 10)]
    n: usize,
}

fn drop_head(num_lines: usize, out: &mut BufWriter<StdoutLock>) -> Result<()> {
    io::stdin().lines().take(num_lines).last();
    let _ = io::copy(&mut io::stdin().lock(), out);
    Ok(())
}

fn drop_tail(num_lines: usize, out: &mut BufWriter<StdoutLock>) -> Result<()> {
    let lines = io::stdin()
        .lines()
        .take(num_lines)
        .filter_map(std::result::Result::ok);
    let mut queue = VecDeque::from_iter(lines);
    for line in io::stdin().lines() {
        queue.push_back(line?);
        let output = queue.pop_front().expect("Expected line in queue");
        writeln!(out, "{}", output)?
    }
    Ok(())
}

#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

pub fn main() -> Result<()> {
    reset_sigpipe();
    let args = Args::parse();
    let mut out = BufWriter::new(io::stdout().lock());
    if args.tail {
        drop_tail(args.n, &mut out)?;
    } else {
        drop_head(args.n, &mut out)?;
    };
    Ok(())
}
