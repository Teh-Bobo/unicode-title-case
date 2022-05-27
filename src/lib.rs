//! This crate is an implementation of the Unicode Title Casing algorithm. It implements a trait
//! on [char] that adds title case handling methods. These methods are very similar to how the std
//! library currently handles uppercase and lowercase.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]
use core::fmt::Write;
use core::iter::FusedIterator;
use std::fmt::{Debug, Formatter};

include!(concat!(env!("OUT_DIR"), "/casing.rs"));

/// Accepts a char and returns the Unicode Title Case for that character as a 3 char array.
///
/// # Examples
/// If the character is already titlecase then it will return itself:
/// ```
/// use unicode_titlecase::to_titlecase;
/// assert_eq!(to_titlecase('A'), ['A', '\0', '\0']);
/// ```
/// Single-char characters are mapped:
/// ```
/// use unicode_titlecase::to_titlecase;
/// assert_eq!(to_titlecase('Ǆ'), ['ǅ', '\0', '\0']);
/// ```
/// Multi-char ligatures are converted:
/// ```
/// use unicode_titlecase::to_titlecase;
/// assert_eq!(to_titlecase('ﬄ'), ['F', 'f', 'l']);
/// ```
/// Locale is ignored:
/// ```
/// use unicode_titlecase::to_titlecase;
/// assert_eq!(to_titlecase('i'), ['I', '\0', '\0']);
/// ```
/// # Locale
/// This function is not locale specific. Unicode special casing has rules for tr and az that
/// this function does not take into account. For tr and az locales use [to_titlecase_tr_or_az]
pub fn to_titlecase(c: char) -> [char; 3] {
    if let Ok(index) = TITLECASE_TABLE.binary_search_by(|&(key, _)| key.cmp(&c)) {
        TITLECASE_TABLE[index].1
    } else {
        [c, '\0', '\0']
    }
}

/// Accepts a char and returns the Unicode Title Case for that character as a 3 char array.
///
/// # Examples
/// If the character is already titlecase then it will return itself:
/// ```
/// use unicode_titlecase::to_titlecase_tr_or_az;
/// assert_eq!(to_titlecase_tr_or_az('A'), ['A', '\0', '\0']);
/// ```
/// Single-char characters are mapped:
/// ```
/// use unicode_titlecase::to_titlecase_tr_or_az;
/// assert_eq!(to_titlecase_tr_or_az('Ǆ'), ['ǅ', '\0', '\0']);
/// ```
/// Multi-char ligatures are converted:
/// ```
/// use unicode_titlecase::to_titlecase_tr_or_az;
/// assert_eq!(to_titlecase_tr_or_az('ﬄ'), ['F', 'f', 'l']);
/// ```
/// Locale is tr/az:
/// ```
/// use unicode_titlecase::to_titlecase_tr_or_az;
/// assert_eq!(to_titlecase_tr_or_az('i'), ['İ', '\0', '\0']);
/// ```
/// # Locale
/// This function is specific to the tr and az locales. It returns different results for certain
/// chars. To use locale agnostic version see [to_titlecase].
pub fn to_titlecase_tr_or_az(c: char) -> [char; 3] {
    if c == '\u{0069}' {
        ['\u{0130}', '\0', '\0']
    } else {
        to_titlecase(c)
    }
}

/// This trait adds title case methods to [char]. They function the same as the std library's
/// [char::to_lowercase] and [char::to_uppercase] using a custom [ToTitleCase] iterator.
pub trait TitleCase {
    /// Wraps [to_titlecase] in an iterator. The iterator will yield at most 3 chars.
    ///
    /// # Examples
    /// If the character is already titlecase then it will return itself
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert_eq!('A'.to_titlecase().to_string(), "A")
    /// ```
    /// Single-char characters are mapped:
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert_eq!('Ǆ'.to_titlecase().to_string(), "ǅ")
    /// ```
    /// Multi-char ligatures are converted:
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert_eq!('ﬄ'.to_titlecase().to_string(), "Ffl")
    /// ```
    /// Locale is ignored:
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert_eq!('i'.to_titlecase().to_string(), "I")
    /// ```
    /// # Locale
    /// This function is not locale specific. Unicode special casing has rules for tr and az that
    /// this function does not take into account. For tr and az locales use [TitleCase::to_titlecase_tr_or_az]
    fn to_titlecase(self) -> ToTitleCase;

    /// Wraps [to_titlecase_tr_or_az] in an iterator. The iterator will yield at most 3 chars.
    ///
    /// # Examples
    /// If the character is already titlecase then it will return itself
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert_eq!('A'.to_titlecase_tr_or_az().to_string(), "A")
    /// ```
    /// Single-char characters are mapped:
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert_eq!('Ǆ'.to_titlecase_tr_or_az().to_string(), "ǅ")
    /// ```
    /// Multi-char ligatures are converted:
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert_eq!('ﬄ'.to_titlecase_tr_or_az().to_string(), "Ffl")
    /// ```
    /// Locale is tr/az:
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert_eq!('i'.to_titlecase_tr_or_az().to_string(), "İ")
    /// ```
    ///
    /// # Locale
    /// This function is specific to the tr and az locales. It returns different results for certain
    /// chars. To use locale agnostic version see [TitleCase::to_titlecase].
    fn to_titlecase_tr_or_az(self) -> ToTitleCase;
}

impl TitleCase for char {
    fn to_titlecase(self) -> ToTitleCase {
        ToTitleCase(CaseMappingIter::new(to_titlecase(self)))
    }

    fn to_titlecase_tr_or_az(self) -> ToTitleCase {
        ToTitleCase(CaseMappingIter::new(to_titlecase_tr_or_az(self)))
    }
}

/// An iterator over a titlecase mapped char.
///
/// Copied from the std library's [core::char::ToLowercase] and [core::char::ToUppercase].
#[derive(Debug, Clone)]
pub struct ToTitleCase(CaseMappingIter);

impl Iterator for ToTitleCase {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for ToTitleCase {
    fn next_back(&mut self) -> Option<char> {
        self.0.next_back()
    }
}

impl FusedIterator for ToTitleCase {}

impl ExactSizeIterator for ToTitleCase {}

#[cfg(feature = "std")]
impl std::fmt::Display for ToTitleCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

// Copied out of the std library
#[derive(Debug, Clone)]
enum CaseMappingIter {
    Three(char, char, char),
    Two(char, char),
    One(char),
    Zero,
}

impl CaseMappingIter {
    fn new(chars: [char; 3]) -> CaseMappingIter {
        if chars[2] == '\0' {
            if chars[1] == '\0' {
                CaseMappingIter::One(chars[0]) // Including if chars[0] == '\0'
            } else {
                CaseMappingIter::Two(chars[0], chars[1])
            }
        } else {
            CaseMappingIter::Three(chars[0], chars[1], chars[2])
        }
    }
}

impl Iterator for CaseMappingIter {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        match *self {
            CaseMappingIter::Three(a, b, c) => {
                *self = CaseMappingIter::Two(b, c);
                Some(a)
            }
            CaseMappingIter::Two(b, c) => {
                *self = CaseMappingIter::One(c);
                Some(b)
            }
            CaseMappingIter::One(c) => {
                *self = CaseMappingIter::Zero;
                Some(c)
            }
            CaseMappingIter::Zero => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = match self {
            CaseMappingIter::Three(..) => 3,
            CaseMappingIter::Two(..) => 2,
            CaseMappingIter::One(_) => 1,
            CaseMappingIter::Zero => 0,
        };
        (size, Some(size))
    }
}

impl DoubleEndedIterator for CaseMappingIter {
    fn next_back(&mut self) -> Option<char> {
        match *self {
            CaseMappingIter::Three(a, b, c) => {
                *self = CaseMappingIter::Two(a, b);
                Some(c)
            }
            CaseMappingIter::Two(b, c) => {
                *self = CaseMappingIter::One(b);
                Some(c)
            }
            CaseMappingIter::One(c) => {
                *self = CaseMappingIter::Zero;
                Some(c)
            }
            CaseMappingIter::Zero => None,
        }
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for CaseMappingIter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            CaseMappingIter::Three(a, b, c) => {
                f.write_char(a)?;
                f.write_char(b)?;
                f.write_char(c)
            }
            CaseMappingIter::Two(b, c) => {
                f.write_char(b)?;
                f.write_char(c)
            }
            CaseMappingIter::One(c) => f.write_char(c),
            CaseMappingIter::Zero => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    include!(concat!(env!("OUT_DIR"), "/casing.rs"));

    #[test]
    fn self_mapping() {
        TITLECASE_TABLE.iter().for_each(|(cp, mapping)| {
            assert_ne!(*cp, mapping[0]);
        });
    }

    #[test]
    fn is_sorted() {
        let mut last = '\0';
        TITLECASE_TABLE.iter().for_each(|(cp, _)| {
            assert!(*cp > last, "cp: {cp}, last: {last}");
            last = *cp;
        });
    }
}
