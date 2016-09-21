# unicode-brackets

Provides methods for determining whether a character is an opening or closing
bracket and for changing the direction of these characters.

The definitions used by this crate are from the unicode bidirectional algorithm
[(UAX #9)](http://unicode.org/reports/tr9/). Specifically, see the file
http://www.unicode.org/Public/UCD/latest/ucd/BidiBrackets.txt

This crate uses the `no_std` attribute which eliminates dependence on `std`.

```rust
extern crate unicode_brackets;
use unicode_brackets::UnicodeBrackets;

fn main() {
    /// Some of the many different kinds of opening bracket.
    let opening_chars = ['(', '[', '⦑'];

    for c in opening_chars {
        assert!(c.is_open_bracket());
    }

    let closing_chars: Vec<char> = opening_chars.iter()
                                                .map(|c| c.to_close_bracket())
                                                .collect();
    assert_eq!(closing_chars[..], [')', ']', '⦒']);
}
```

