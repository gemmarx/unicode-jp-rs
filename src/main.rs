
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate clap;
extern crate kana;

use std::error::Error;
use std::{io, process, fs};
use std::io::prelude::*;
use clap::ArgMatches;
use kana::*;

macro_rules! err { ($e:expr) => ( {
    writeln!(&mut io::stderr(), "{}", $e).unwrap();
    process::exit(1);
} ) }

fn main() {
    let _args = load_yaml!("cli.yml");
    let args  = clap::App::from_yaml(_args).get_matches();

    match main_body(&args, get_input_clap(&args)) {
        Ok(_)  => {},
        Err(e) => err!(e),
    }
}

fn main_body(args: &ArgMatches, input: Box<BufRead>)
    -> Result<(), Box<Error>>
{
    let k = Kana::init();
    for _s in input.lines() {
        let mut s = try!(_s);
        if args.is_present("half2full")    { s = k.half2full(&s); }
        if args.is_present("half2kana")    { s = k.half2kana(&s); }
        if args.is_present("combine")      { s = k.combine(&s); }
        if args.is_present("hira2kata")    { s = hira2kata(&s); }
        if args.is_present("kata2hira")    { s = kata2hira(&s); }
        if args.is_present("vsmark2half")  { s = vsmark2half(&s); }
        if args.is_present("vsmark2full")  { s = vsmark2full(&s); }
        if args.is_present("vsmark2combi") { s = vsmark2combi(&s); }
        if args.is_present("ascii2wide")   { s = ascii2wide(&s); }
        if args.is_present("wide2ascii")   { s = wide2ascii(&s); }
        if args.is_present("nowidespace")  { s = nowidespace(&s); }
        if args.is_present("space2wide")   { s = space2wide(&s); }
        if args.is_present("nowideyen")    { s = nowideyen(&s); }
        if args.is_present("yen2wide")     { s = yen2wide(&s); }
        println!("{}", s);
    }
    Ok(())
}

fn get_input_clap(args: &ArgMatches) -> Box<BufRead> {
    if args.is_present("INPUT") {
        let f = fs::File::open(args.value_of("INPUT").unwrap())
                .unwrap_or_else(|e|err!(e));
        Box::new(io::BufReader::new(f)) as Box<BufRead>
    } else {
        lazy_static! { static ref STDIN: io::Stdin = io::stdin(); }
        Box::new(STDIN.lock()) as Box<BufRead>
    }
}

