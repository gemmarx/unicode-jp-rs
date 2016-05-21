
extern crate kana;

use kana::Kana;

fn main() {
    let k = Kana::init();
    let r = std::io::stdin();

    let buff = &mut String::new();
    while r.read_line(buff).is_ok() && 0 != buff.len() {
        print!("{}", k.half2kana(buff));
        buff.clear();
    }
}

