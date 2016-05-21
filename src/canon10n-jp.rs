
extern crate kana;

use kana::{Kana, wide2ascii, vsmark2combi};

fn main() {
    let k = Kana::init();
    let r = std::io::stdin();

    let buff = &mut String::new();
    while r.read_line(buff).is_ok() && 0 != buff.len() {
        let s = wide2ascii(&vsmark2combi(&k.combine(&k.half2full(buff))));
        print!("{}", s.replace("\u{3000}", "\u{20}"));
        buff.clear();
    }
}

