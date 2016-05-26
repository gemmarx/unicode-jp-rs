
#[macro_use] extern crate clap;
extern crate kana;

use clap::App;
use kana::Kana;

fn main() {
    let clis = load_yaml!("cli.yml");
    let args = App::from_yaml(clis).get_matches();

    let k = Kana::init();
    let r = std::io::stdin();

    let buff = &mut String::new();
    while r.read_line(buff).is_ok() && 0 != buff.len() {
        if args.is_present("ascii2wide") {
            print!("{}", kana::ascii2wide(buff));
        } else if args.is_present("wide2ascii") {
            print!("{}", kana::wide2ascii(buff));
        } else if args.is_present("hira2kata") {
            print!("{}", kana::hira2kata(buff));
        } else if args.is_present("kata2hira") {
            print!("{}", kana::kata2hira(buff));
        } else if args.is_present("combine") {
            print!("{}", k.combine(buff));
        } else if args.is_present("half2full") {
            print!("{}", k.half2full(buff));
        } else if args.is_present("half2kana") {
            print!("{}", k.half2kana(buff));
        } else if args.is_present("vsmark2half") {
            print!("{}", kana::vsmark2half(buff));
        } else if args.is_present("vsmark2full") {
            print!("{}", kana::vsmark2full(buff));
        } else if args.is_present("vsmark2combi") {
            print!("{}", kana::vsmark2combi(buff));
        } else if args.is_present("nowidespace") {
            print!("{}", buff.replace("\u{3000}", "\u{20}"));
        } else {
            print!("{}", buff);
        }
        buff.clear();
    }
}

