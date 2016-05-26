Unicode-JP
----
Converters of characters included in Japanese texts.
- Half-width-kana[半角ｶﾅ;HANKAKU KANA] -> normal Katakana
- Wide-alphanumeric[全角英数;ZENKAKU EISU] <-> normal ASCII

If you need canonicalization of text including Japanese, consider to use [unicode_normalization](https://github.com/unicode-rs/unicode-normalization) crate at first.
NFD, NFKD, NFC and NFKC can be used.
This crate, however, works with you if you are on a niche such as a need of delicate control of Japanese characters for a restrictive character terminal.

Japanese have two syllabary systems Hiragana and Katakana, and Half-width-kana is another notation system of them.
In the systems, there are two combinable diacritical marks Voiced-sound-mark and Semi-voiced-sound-mark.
Unicode has three independent code points for each of the marks.
In addition to it, we often use special style Latin alphabets and Arabic numbers called Wide-alphanumeric in Japanese texts.
This small utility converts these codes each other.

### Example
Cargo.toml
```toml
[dependencies]
unicode-jp = { git = "https://github.com/gemmarx/unicode-jp-rs" }
```

src/main.rs
```rust
extern crate kana;
use kana::Kana;

fn main() {
    let k = Kana::init();

    let s1 = "ﾏﾂｵ ﾊﾞｼｮｳ ｱﾟ";
    assert_eq!("マツオ バショウ ア ゚", k.half2kana(s1));
    assert_eq!("マツオ バショウ ア゚", k.half2full(s1));

    let s2 = "ひ゜ひ゛んは゛";
    assert_eq!("ぴびんば", k.combine(s2));
    assert_eq!("ひ ゚ひ ゙んは ゙", kana::vsmark2combi(s2));

    let s3 = "＃＆Ｒｕｓｔ－１．６！";
    assert_eq!("#&Rust-1.6!", kana::wide2ascii(s3));
}
```

### Functions of kana crate:
- wide2ascii(&str) -> String  
convert Wide-alphanumeric into normal ASCII  [Ａ -> A]

- ascii2wide(&str) -> String  
convert normal ASCII characters into Wide-alphanumeric  [A -> Ａ]

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

### Methods of kana::Kana struct:
- half2full(&self, &str) -> String  
convert Half-width-kana into normal Katakana with diacritical marks separated  [ｱﾞﾊﾟ -> ア゙パ]  
This is simple but tends to cause troubles when rendering.
In such a case, use half2kana() or execute vsmark2* as post process.

- half2kana(&self, &str) -> String  
convert Half-width-kana into normal Katakana with diacritical marks combined  [ｱﾞﾊﾟ -> アﾞパ]

- combine(&self, &str) -> String  
combine base characters and diacritical marks on Hiragana/Katakana [かﾞハ゜ -> がパ]

## TODO or NOT TODO
- Half-width-kana <- normal Katakana    # Isn't it need?
- Wide-space "\u{3000}" <-> normal space "\u{20}"   # Use replace() of std::str.

