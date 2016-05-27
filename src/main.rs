
#[macro_use] extern crate clap;
extern crate kana;

use std::io::prelude::*;
use std::{io, process, fs};

macro_rules! err { () => { |e| {
    writeln!(&mut io::stderr(), "{}", e).unwrap();
    process::exit(1);
} };}

fn main() {
    let clis = load_yaml!("cli.yml");
    let args = clap::App::from_yaml(clis).get_matches();

    let f0 = io::stdin();
    let r = if args.is_present("INPUT_FILE") {
        let f = fs::File::open(args.value_of("INPUT_FILE").unwrap()).unwrap_or_else(err![]);
        Box::new(io::BufReader::new(f)) as Box<BufRead>
    } else { Box::new(f0.lock()) as Box<BufRead> };

    let k = kana::Kana::init();
    for line in r.lines() {
        let mut s = line.unwrap();
        if args.is_present("half2full")    { s = k.half2full(&s); }
        if args.is_present("half2kana")    { s = k.half2kana(&s); }
        if args.is_present("combine")      { s = k.combine(&s); }
        if args.is_present("hira2kata")    { s = kana::hira2kata(&s); }
        if args.is_present("kata2hira")    { s = kana::kata2hira(&s); }
        if args.is_present("vsmark2half")  { s = kana::vsmark2half(&s); }
        if args.is_present("vsmark2full")  { s = kana::vsmark2full(&s); }
        if args.is_present("vsmark2combi") { s = kana::vsmark2combi(&s); }
        if args.is_present("ascii2wide")   { s = kana::ascii2wide(&s); }
        if args.is_present("wide2ascii")   { s = kana::wide2ascii(&s); }
        if args.is_present("nowidespace")  { s = s.replace("\u{3000}", "\u{20}"); }
        println!("{}", s);
    }
}

