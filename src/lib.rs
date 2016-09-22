//! Provides methods for determining whether a character is an opening or closing bracket and for
//! changing the direction of these characters.
//!
//! The definitions used in this crate are from the unicode bidirectional algorithm
//! [(UAX #9)](http://unicode.org/reports/tr9/). Specifically, see the file
//! http://www.unicode.org/Public/UCD/latest/ucd/BidiBrackets.txt
//!
//! This crate uses the `no_std` attribute which eliminates dependence on `std`.
//!
//! ```rust
//! extern crate unicode_brackets;
//! use unicode_brackets::UnicodeBrackets;
//!
//! fn main() {
//!     // Some of the many different kinds of opening bracket.
//!     let opening_chars = ['(', '[', '⦑'];
//!
//!     for c in opening_chars.iter() {
//!         assert!(c.is_open_bracket());
//!     }
//!
//!     let closing_chars: Vec<char> = opening_chars.iter()
//!                                                 .map(|c| c.to_close_bracket())
//!                                                 .collect();
//!     assert_eq!(closing_chars[..], [')', ']', '⦒']);
//! }
//! ```

#![deny(missing_docs, unsafe_code)]
#![no_std]

/// The version of [Unicode](http://www.unicode.org/) that this version of unicode-brackets is
/// based on.
pub const UNICODE_VERSION: (u64, u64, u64) = (9, 0, 0);

/// Methods for determining whether a character is an opening or closing bracket and for changing
/// the direction of such characters.
///
/// The definitions used in this crate are from the unicode bidirectional algorithm
/// [(UAX #9)](http://unicode.org/reports/tr9/). Specifically, see the file
/// http://www.unicode.org/Public/UCD/latest/ucd/BidiBrackets.txt
pub trait UnicodeBrackets: Eq {
    /// Determine whether a character is an opening bracket.
    fn is_open_bracket(&self) -> bool {
        self.to_close_bracket() != *self
    }

    /// Determine whether a character is a closing bracket.
    fn is_close_bracket(&self) -> bool {
        self.to_open_bracket() != *self
    }

    /// Convert a closing bracket character to an opening bracket. Returns `self` if the character
    /// is not a closing bracket.
    fn to_open_bracket(&self) -> Self;

    /// Convert an opening bracket character to a closing bracket. Returns `self` if the character
    /// is not an opening bracket.
    fn to_close_bracket(&self) -> Self;
}

impl UnicodeBrackets for char {
    fn to_close_bracket(&self) -> char {
        match *self {
            '\u{0028}' => '\u{0029}', // LEFT PARENTHESIS
            '\u{005B}' => '\u{005D}', // LEFT SQUARE BRACKET
            '\u{007B}' => '\u{007D}', // LEFT CURLY BRACKET
            '\u{0F3A}' => '\u{0F3B}', // TIBETAN MARK GUG RTAGS GYON
            '\u{0F3C}' => '\u{0F3D}', // TIBETAN MARK ANG KHANG GYON
            '\u{169B}' => '\u{169C}', // OGHAM FEATHER MARK
            '\u{2045}' => '\u{2046}', // LEFT SQUARE BRACKET WITH QUILL
            '\u{207D}' => '\u{207E}', // SUPERSCRIPT LEFT PARENTHESIS
            '\u{208D}' => '\u{208E}', // SUBSCRIPT LEFT PARENTHESIS
            '\u{2308}' => '\u{2309}', // LEFT CEILING
            '\u{230A}' => '\u{230B}', // LEFT FLOOR
            '\u{2329}' => '\u{232A}', // LEFT-POINTING ANGLE BRACKET
            '\u{2768}' => '\u{2769}', // MEDIUM LEFT PARENTHESIS ORNAMENT
            '\u{276A}' => '\u{276B}', // MEDIUM FLATTENED LEFT PARENTHESIS ORNAMENT
            '\u{276C}' => '\u{276D}', // MEDIUM LEFT-POINTING ANGLE BRACKET ORNAMENT
            '\u{276E}' => '\u{276F}', // HEAVY LEFT-POINTING ANGLE QUOTATION MARK ORNAMENT
            '\u{2770}' => '\u{2771}', // HEAVY LEFT-POINTING ANGLE BRACKET ORNAMENT
            '\u{2772}' => '\u{2773}', // LIGHT LEFT TORTOISE SHELL BRACKET ORNAMENT
            '\u{2774}' => '\u{2775}', // MEDIUM LEFT CURLY BRACKET ORNAMENT
            '\u{27C5}' => '\u{27C6}', // LEFT S-SHAPED BAG DELIMITER
            '\u{27E6}' => '\u{27E7}', // MATHEMATICAL LEFT WHITE SQUARE BRACKET
            '\u{27E8}' => '\u{27E9}', // MATHEMATICAL LEFT ANGLE BRACKET
            '\u{27EA}' => '\u{27EB}', // MATHEMATICAL LEFT DOUBLE ANGLE BRACKET
            '\u{27EC}' => '\u{27ED}', // MATHEMATICAL LEFT WHITE TORTOISE SHELL BRACKET
            '\u{27EE}' => '\u{27EF}', // MATHEMATICAL LEFT FLATTENED PARENTHESIS
            '\u{2983}' => '\u{2984}', // LEFT WHITE CURLY BRACKET
            '\u{2985}' => '\u{2986}', // LEFT WHITE PARENTHESIS
            '\u{2987}' => '\u{2988}', // Z NOTATION LEFT IMAGE BRACKET
            '\u{2989}' => '\u{298A}', // Z NOTATION LEFT BINDING BRACKET
            '\u{298B}' => '\u{298C}', // LEFT SQUARE BRACKET WITH UNDERBAR
            '\u{298D}' => '\u{2990}', // LEFT SQUARE BRACKET WITH TICK IN TOP CORNER
            '\u{298F}' => '\u{298E}', // LEFT SQUARE BRACKET WITH TICK IN BOTTOM CORNER
            '\u{2991}' => '\u{2992}', // LEFT ANGLE BRACKET WITH DOT
            '\u{2993}' => '\u{2994}', // LEFT ARC LESS-THAN BRACKET
            '\u{2995}' => '\u{2996}', // DOUBLE LEFT ARC GREATER-THAN BRACKET
            '\u{2997}' => '\u{2998}', // LEFT BLACK TORTOISE SHELL BRACKET
            '\u{29D8}' => '\u{29D9}', // LEFT WIGGLY FENCE
            '\u{29DA}' => '\u{29DB}', // LEFT DOUBLE WIGGLY FENCE
            '\u{29FC}' => '\u{29FD}', // LEFT-POINTING CURVED ANGLE BRACKET
            '\u{2E22}' => '\u{2E23}', // TOP LEFT HALF BRACKET
            '\u{2E24}' => '\u{2E25}', // BOTTOM LEFT HALF BRACKET
            '\u{2E26}' => '\u{2E27}', // LEFT SIDEWAYS U BRACKET
            '\u{2E28}' => '\u{2E29}', // LEFT DOUBLE PARENTHESIS
            '\u{3008}' => '\u{3009}', // LEFT ANGLE BRACKET
            '\u{300A}' => '\u{300B}', // LEFT DOUBLE ANGLE BRACKET
            '\u{300C}' => '\u{300D}', // LEFT CORNER BRACKET
            '\u{300E}' => '\u{300F}', // LEFT WHITE CORNER BRACKET
            '\u{3010}' => '\u{3011}', // LEFT BLACK LENTICULAR BRACKET
            '\u{3014}' => '\u{3015}', // LEFT TORTOISE SHELL BRACKET
            '\u{3016}' => '\u{3017}', // LEFT WHITE LENTICULAR BRACKET
            '\u{3018}' => '\u{3019}', // LEFT WHITE TORTOISE SHELL BRACKET
            '\u{301A}' => '\u{301B}', // LEFT WHITE SQUARE BRACKET
            '\u{FE59}' => '\u{FE5A}', // SMALL LEFT PARENTHESIS
            '\u{FE5B}' => '\u{FE5C}', // SMALL LEFT CURLY BRACKET
            '\u{FE5D}' => '\u{FE5E}', // SMALL LEFT TORTOISE SHELL BRACKET
            '\u{FF08}' => '\u{FF09}', // FULLWIDTH LEFT PARENTHESIS
            '\u{FF3B}' => '\u{FF3D}', // FULLWIDTH LEFT SQUARE BRACKET
            '\u{FF5B}' => '\u{FF5D}', // FULLWIDTH LEFT CURLY BRACKET
            '\u{FF5F}' => '\u{FF60}', // FULLWIDTH LEFT WHITE PARENTHESIS
            '\u{FF62}' => '\u{FF63}', // HALFWIDTH LEFT CORNER BRACKET
            _ => *self,
        }
    }

    fn to_open_bracket(&self) -> char {
        match *self {
            '\u{0029}' => '\u{0028}', // RIGHT PARENTHESIS
            '\u{005D}' => '\u{005B}', // RIGHT SQUARE BRACKET
            '\u{007D}' => '\u{007B}', // RIGHT CURLY BRACKET
            '\u{0F3B}' => '\u{0F3A}', // TIBETAN MARK GUG RTAGS GYAS
            '\u{0F3D}' => '\u{0F3C}', // TIBETAN MARK ANG KHANG GYAS
            '\u{169C}' => '\u{169B}', // OGHAM REVERSED FEATHER MARK
            '\u{2046}' => '\u{2045}', // RIGHT SQUARE BRACKET WITH QUILL
            '\u{207E}' => '\u{207D}', // SUPERSCRIPT RIGHT PARENTHESIS
            '\u{208E}' => '\u{208D}', // SUBSCRIPT RIGHT PARENTHESIS
            '\u{2309}' => '\u{2308}', // RIGHT CEILING
            '\u{230B}' => '\u{230A}', // RIGHT FLOOR
            '\u{232A}' => '\u{2329}', // RIGHT-POINTING ANGLE BRACKET
            '\u{2769}' => '\u{2768}', // MEDIUM RIGHT PARENTHESIS ORNAMENT
            '\u{276B}' => '\u{276A}', // MEDIUM FLATTENED RIGHT PARENTHESIS ORNAMENT
            '\u{276D}' => '\u{276C}', // MEDIUM RIGHT-POINTING ANGLE BRACKET ORNAMENT
            '\u{276F}' => '\u{276E}', // HEAVY RIGHT-POINTING ANGLE QUOTATION MARK ORNAMENT
            '\u{2771}' => '\u{2770}', // HEAVY RIGHT-POINTING ANGLE BRACKET ORNAMENT
            '\u{2773}' => '\u{2772}', // LIGHT RIGHT TORTOISE SHELL BRACKET ORNAMENT
            '\u{2775}' => '\u{2774}', // MEDIUM RIGHT CURLY BRACKET ORNAMENT
            '\u{27C6}' => '\u{27C5}', // RIGHT S-SHAPED BAG DELIMITER
            '\u{27E7}' => '\u{27E6}', // MATHEMATICAL RIGHT WHITE SQUARE BRACKET
            '\u{27E9}' => '\u{27E8}', // MATHEMATICAL RIGHT ANGLE BRACKET
            '\u{27EB}' => '\u{27EA}', // MATHEMATICAL RIGHT DOUBLE ANGLE BRACKET
            '\u{27ED}' => '\u{27EC}', // MATHEMATICAL RIGHT WHITE TORTOISE SHELL BRACKET
            '\u{27EF}' => '\u{27EE}', // MATHEMATICAL RIGHT FLATTENED PARENTHESIS
            '\u{2984}' => '\u{2983}', // RIGHT WHITE CURLY BRACKET
            '\u{2986}' => '\u{2985}', // RIGHT WHITE PARENTHESIS
            '\u{2988}' => '\u{2987}', // Z NOTATION RIGHT IMAGE BRACKET
            '\u{298A}' => '\u{2989}', // Z NOTATION RIGHT BINDING BRACKET
            '\u{298C}' => '\u{298B}', // RIGHT SQUARE BRACKET WITH UNDERBAR
            '\u{298E}' => '\u{298F}', // RIGHT SQUARE BRACKET WITH TICK IN BOTTOM CORNER
            '\u{2990}' => '\u{298D}', // RIGHT SQUARE BRACKET WITH TICK IN TOP CORNER
            '\u{2992}' => '\u{2991}', // RIGHT ANGLE BRACKET WITH DOT
            '\u{2994}' => '\u{2993}', // RIGHT ARC GREATER-THAN BRACKET
            '\u{2996}' => '\u{2995}', // DOUBLE RIGHT ARC LESS-THAN BRACKET
            '\u{2998}' => '\u{2997}', // RIGHT BLACK TORTOISE SHELL BRACKET
            '\u{29D9}' => '\u{29D8}', // RIGHT WIGGLY FENCE
            '\u{29DB}' => '\u{29DA}', // RIGHT DOUBLE WIGGLY FENCE
            '\u{29FD}' => '\u{29FC}', // RIGHT-POINTING CURVED ANGLE BRACKET
            '\u{2E23}' => '\u{2E22}', // TOP RIGHT HALF BRACKET
            '\u{2E25}' => '\u{2E24}', // BOTTOM RIGHT HALF BRACKET
            '\u{2E27}' => '\u{2E26}', // RIGHT SIDEWAYS U BRACKET
            '\u{2E29}' => '\u{2E28}', // RIGHT DOUBLE PARENTHESIS
            '\u{3009}' => '\u{3008}', // RIGHT ANGLE BRACKET
            '\u{300B}' => '\u{300A}', // RIGHT DOUBLE ANGLE BRACKET
            '\u{300D}' => '\u{300C}', // RIGHT CORNER BRACKET
            '\u{300F}' => '\u{300E}', // RIGHT WHITE CORNER BRACKET
            '\u{3011}' => '\u{3010}', // RIGHT BLACK LENTICULAR BRACKET
            '\u{3015}' => '\u{3014}', // RIGHT TORTOISE SHELL BRACKET
            '\u{3017}' => '\u{3016}', // RIGHT WHITE LENTICULAR BRACKET
            '\u{3019}' => '\u{3018}', // RIGHT WHITE TORTOISE SHELL BRACKET
            '\u{301B}' => '\u{301A}', // RIGHT WHITE SQUARE BRACKET
            '\u{FE5A}' => '\u{FE59}', // SMALL RIGHT PARENTHESIS
            '\u{FE5C}' => '\u{FE5B}', // SMALL RIGHT CURLY BRACKET
            '\u{FE5E}' => '\u{FE5D}', // SMALL RIGHT TORTOISE SHELL BRACKET
            '\u{FF09}' => '\u{FF08}', // FULLWIDTH RIGHT PARENTHESIS
            '\u{FF3D}' => '\u{FF3B}', // FULLWIDTH RIGHT SQUARE BRACKET
            '\u{FF5D}' => '\u{FF5B}', // FULLWIDTH RIGHT CURLY BRACKET
            '\u{FF60}' => '\u{FF5F}', // FULLWIDTH RIGHT WHITE PARENTHESIS
            '\u{FF63}' => '\u{FF62}', // HALFWIDTH RIGHT CORNER BRACKET
            _ => *self,
        }
    }
}

