
extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;

fn main() {
    let reader = std::io::stdin();
    let buff = &mut String::new();
    while reader.read_line(buff).is_ok() && 0 != buff.len() {
        print!("{}", buff.nfkc().collect::<String>());
        buff.clear();
    }
}

