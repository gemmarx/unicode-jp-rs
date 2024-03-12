Temporary Crate
____
This crate exists due to https://github.com/gemmarx/unicode-jp-rs appearing inactive, and updates needed. Please contact me and I will yank my versions from crates.io if needed.

Unicode-JP (Rust)
----
[![Build Status](https://travis-ci.org/gemmarx/unicode-jp-rs.svg?branch=master)](https://travis-ci.org/gemmarx/unicode-jp-rs)
[![crates.io](https://img.shields.io/crates/v/unicode-jp.svg)](https://crates.io/crates/unicode-jp)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Converters of troublesome characters included in Japanese texts.
- Half-width-kana[半角ｶﾅ;HANKAKU KANA] -> normal Katakana
- Wide-alphanumeric[全角英数;ZENKAKU EISU] <-> normal ASCII

If you need canonicalization of texts including Japanese, consider to use [unicode_normalization](https://github.com/unicode-rs/unicode-normalization) crate at first.
NFD, NFKD, NFC and NFKC can be used.
This crate, however, works with you if you are in a niche such as a need of delicate control of Japanese characters for a restrictive character terminal.

Japanese have two syllabary systems Hiragana and Katakana, and Half-width-kana is another notation system of them.
In the systems, there are two combinable diacritical marks Voiced-sound-mark and Semi-voiced-sound-mark.
Unicode has three independent code points for each of the marks.
In addition to it, we often use special style Latin alphabets and Arabic numbers called Wide-alphanumeric in Japanese texts.
This small utility converts these codes each other.

[API Reference](https://gemmarx.github.io/unicode-jp-rs/doc/kana/index.html)

### Example
Cargo.toml
```toml
[dependencies]
unicode-jp = "0.4.0"
```

src/main.rs
```rust
extern crate kana;
use kana::*;

fn main() {
    let s1 = "ﾏﾂｵ ﾊﾞｼｮｳ ｱﾟ";
    assert_eq!("マツオ バショウ ア ゚", half2kana(s1));
    assert_eq!("マツオ バショウ ア゚", half2full(s1));

    let s2 = "ひ゜ひ゛んは゛";
    assert_eq!("ぴびんば", combine(s2));
    assert_eq!("ひ ゚ひ ゙んは ゙", vsmark2combi(s2));

    let s3 = "＃＆Ｒｕｓｔ－１．６！";
    assert_eq!("#&Rust-1.6!", wide2ascii(s3));
}
```

### Functions of kana crate:
- wide2ascii(&str) -> String  
convert Wide-alphanumeric into normal ASCII  [Ａ -> A]

- ascii2wide(&str) -> String  
convert normal ASCII characters into Wide-alphanumeric  [A -> Ａ]

- half2full(&str) -> String  
convert Half-width-kana into normal Katakana with diacritical marks separated  [ｱﾞﾊﾟ -> ア゙パ]  
This method is simple, but tends to cause troubles when rendering.
In such a case, use half2kana() or execute vsmark2{full|half|combi} as post process.

- half2kana(&str) -> String  
convert Half-width-kana into normal Katakana with diacritical marks combined  [ｱﾞﾊﾟ -> アﾞパ]

- combine(&str) -> String  
combine base characters and diacritical marks on Hiragana/Katakana [かﾞハ゜ -> がパ]

- hira2kata(&str) -> String  
convert Hiragana into Katakana  [あ -> ア]

- kata2hira(&str) -> String  
convert Katakana into Hiragana  [ア -> あ]

- vsmark2full(&str) -> String  
convert all separated Voiced-sound-marks into full-width style "\u{309B}"

- vsmark2half(&str) -> String  
convert all separated Voiced-sound-marks into half-width style "\u{FF9E}"

- vsmark2combi(&str) -> String  
convert all separated Voiced-sound-marks into space+combining style "\u{20}\u{3099}"

- nowidespace(&str) -> String  
convert Wide-space into normal space    ["　" -> " "]

- space2wide(&str) -> String  
convert normal space into Wide-space    [" " -> "　"]

- nowideyen(&str) -> String  
convert Wide-yen into Half-width-yen    ["￥" -> "¥"]

- yen2wide(&str) -> String  
convert Half-width-yen into Wide-yen    ["¥" -> "￥"]

## TODO or NOT TODO
- Voiced-sound-marks -> no space combining style "\u{3099}"
- Half-width-kana <- normal Katakana
- (normal/wide)tilde <-> Wave-dash

