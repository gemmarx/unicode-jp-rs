mod kana;

use clap::Parser;
use kana::*;
use std::error::Error;
use std::io::prelude::*;
use std::{fs, io, process};

macro_rules! err {
    ($e:expr) => {{
        writeln!(&mut io::stderr(), "{}", $e).unwrap();
        process::exit(1);
    }};
}

macro_rules! command {
    ($($name:ident,)*) => {
        #[derive(Parser, Debug)]
        pub struct Command {
            $(
                #[arg(long)]
                $name: bool,
            )*
            input: Option<String>,
        }
    };
}

command!(
    half2full,
    half2kana,
    combine,
    hira2kata,
    kata2hira,
    vsmark2half,
    vsmark2full,
    vsmark2combi,
    ascii2wide,
    wide2ascii,
    nowidespace,
    space2wide,
    nowideyen,
    yen2wide,
);

fn main() {
    let args = Command::parse();
    match main_body(&args, get_input_clap(&args)) {
        Ok(_) => {}
        Err(e) => err!(e),
    }
}

macro_rules! get_converter_macro {
    ($args:ident, $($name:ident,)*) => {
        $(
            if $args.$name {
                return $name;
            }
        )*
    };
}

fn get_converter(args: &Command) -> for<'a> fn(&'a str) -> String {
    get_converter_macro!(
        args,
        half2full,
        half2kana,
        combine,
        hira2kata,
        kata2hira,
        vsmark2half,
        vsmark2full,
        vsmark2combi,
        ascii2wide,
        wide2ascii,
        nowidespace,
        space2wide,
        nowideyen,
        yen2wide,
    );
    panic!();
}

fn main_body(args: &Command, input: Input) -> Result<(), Box<dyn Error>> {
    let converter = get_converter(args);

    for _s in input.lines() {
        let s = _s?;
        println!("{}", converter(&converter(&s)));
    }
    Ok(())
}

fn get_input_clap(args: &Command) -> Input {
    if let Some(input) = &&args.input {
        let f = fs::File::open(input).unwrap_or_else(|e| err!(e));
        Input::BufReader(io::BufReader::new(f))
    } else {
        let stdin = io::stdin();
        let lock = stdin.lock();
        Input::Stdio {
            stdin,
            lock,
        }
    }
}

pub enum Input {
    BufReader(io::BufReader<fs::File>),
    Stdio {
        stdin: io::Stdin,
        lock: io::StdinLock<'static>,
    },
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::BufReader(reader) => reader.read(buf),
            Self::Stdio { stdin, .. } => stdin.read(buf),
        }
    }
}

impl BufRead for Input {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            Self::BufReader(reader) => reader.fill_buf(),
            Self::Stdio { lock, .. } => lock.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            Self::BufReader(reader) => reader.consume(amt),
            Self::Stdio { lock, .. } => lock.consume(amt),
        }
    }
}
