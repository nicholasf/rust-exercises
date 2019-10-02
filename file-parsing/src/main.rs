use std::env;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        println!("\n\n Please supply a search term!");
        process::exit(0);
    }
    
    let term: &str = args[1].trim();
    parse(&*term);
}

fn parse(term: &str) -> io::Result<()> {
    let cwd = env::current_dir()?;
    let ulysses = cwd.join("data/ulysses.txt");
    let f = File::open(ulysses)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let l = line?;

        if l.contains(term) {
            println!("{}", l);
        }
    }

    Ok(())
}

