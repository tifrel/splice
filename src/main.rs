// use regex::Regex;
use std::io::prelude::*;
use std::io::Error as IOError; /* BufReader.lines() */
// use std::io::{BufRead, Write}; /* BufReader::lines() */
mod cfg;
use cfg::*;
mod errors;
use errors::Error;

fn main() {
    std::process::exit(match splice() {
        Ok(()) => 0,
        Err(e) => {
            println!("{}", e);
            e.code
        }
    })
}

// TODO:
//  [] handling of cutting sites (output or not?)
//  [] regex support
//  [] choice between all introns or only the first/last n
//  [] optimization potential: when there is only one intron -> just plug r into w
//      - impl via 2 bools (found, finished?) and is extendable for Vec<Intron> via Vec<(bool,bool)>
//      - or a struct Intron {name: String, found: bool, finished: bool}
type SpliceResult = Result<(), IOError>;
// type SpliceResult = Result<(), Box<dyn Into>;

fn splice() -> Result<(), Error> {
    let mut cfg: Config = cfg::read_args()?;
    // get reader from config
    let r = cfg.get_reader()?;
    // // get writer from config
    let mut w = cfg.get_writer()?;

    // let the main modes take over
    // can I make a closure out of my desired function and use if let?
    if cfg.exons {
        read_exons(r, &mut w, &cfg)?;
    } else if cfg.insert.is_some() {
        sub_intron(r, &mut w, &cfg)?;
    } else {
        read_introns(r, &mut w, &cfg)?;
    }

    cfg.finish()?;
    Ok(())
}

fn read_exons<R, W>(r: R, w: &mut W, cfg: &Config) -> SpliceResult
where
    R: BufRead,
    W: Write,
{
    let mut is_exon = true;
    for l in r.lines() {
        let line = l.unwrap();
        // checking for stop
        is_exon |= line.contains(&cfg.stop_codon);
        if is_exon {
            // checking for start
            is_exon &= !line.contains(&cfg.start_codon);
            writeln!(w, "{}", line)?;
        }
    }
    Ok(())
}

fn sub_intron<R, W>(r: R, w: &mut W, cfg: &Config) -> SpliceResult
where
    R: BufRead,
    W: Write,
{
    let mut is_exon = true;
    let mut printed = false;
    for l in r.lines() {
        let line = l.unwrap();
        // checking for stop
        is_exon |= line.contains(&cfg.stop_codon);
        if is_exon {
            // checking for start
            is_exon &= !line.contains(&cfg.start_codon);
            printed = false;
            writeln!(w, "{}", line)?;
        }
        if !is_exon && !printed {
            // unwrap ok because this branch is only taken when its not empty
            writeln!(w, "{}", cfg.insert.as_ref().unwrap())?;
            printed = true;
        }
    }
    Ok(())
}

fn read_introns<R, W>(r: R, w: &mut W, cfg: &Config) -> SpliceResult
where
    R: BufRead,
    W: Write,
{
    let mut is_intron = false;
    for l in r.lines() {
        let line = l.unwrap();
        // checking for start
        is_intron |= line.contains(&cfg.start_codon);
        if is_intron {
            // checking for stop
            is_intron &= !line.contains(&cfg.stop_codon);
            // writing
            writeln!(w, "{}", line)?;
        }
    }
    Ok(())
}
