//! This crate is an implementation of the Unicode Title Casing algorithm. It implements a trait
//! on [char] and [str] that adds title case handling methods. These methods are very similar to how
//! the std library currently handles uppercase and lowercase.

#![no_std]
// #![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]
#![deny(unsafe_code)]
#![warn(clippy::pedantic)]

extern crate alloc;

use alloc::string::String;
use core::fmt::{Debug, Display, Formatter, Result, Write};
use core::iter::FusedIterator;

use crate::tr_az::to_lowercase_tr_or_az;

include!(concat!(env!("OUT_DIR"), "/casing.rs"));

#[allow(clippy::doc_link_with_quotes)]
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
/// this function does not take into account. For tr and az locales use [`to_titlecase_tr_or_az`]
#[must_use]
pub fn to_titlecase(c: char) -> [char; 3] {
    if let Ok(index) = TITLECASE_TABLE.binary_search_by(|&(key, _)| key.cmp(&c)) {
        TITLECASE_TABLE[index].1
    } else {
        [c, '\0', '\0']
    }
}

#[allow(clippy::doc_link_with_quotes)]
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
/// chars. To use locale agnostic version see [`to_titlecase`].
#[must_use]
pub fn to_titlecase_tr_or_az(c: char) -> [char; 3] {
    if c == '\u{0069}' {
        ['\u{0130}', '\0', '\0']
    } else {
        to_titlecase(c)
    }
}

/// This trait adds title case methods to [`char`]. They function the same as the std library's
/// [`char::to_lowercase`] and [`char::to_uppercase`] using a custom [`ToTitleCase`] iterator.
pub trait TitleCase {
    /// Wraps [`to_titlecase`] in an iterator. The iterator will yield at most 3 chars.
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
    /// this function does not take into account. For tr and az locales use [`TitleCase::to_titlecase_tr_or_az`]
    fn to_titlecase(self) -> ToTitleCase;

    /// Wraps [`to_titlecase_tr_or_az`] in an iterator. The iterator will yield at most 3 chars.
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
    /// chars. To use locale agnostic version see [`TitleCase::to_titlecase`].
    fn to_titlecase_tr_or_az(self) -> ToTitleCase;

    /// Returns true if the given character is a titlecase character. This function works for all locales
    /// including tr and az.
    /// # Examples
    /// ```
    /// use unicode_titlecase::TitleCase;
    /// assert!('A'.is_titlecase());
    /// assert!('ǅ'.is_titlecase());
    /// assert!('İ'.is_titlecase());
    ///
    /// assert!(!'a'.is_titlecase());
    /// assert!(!'Ǆ'.is_titlecase());
    /// assert!(!'ﬄ'.is_titlecase());
    /// ```
    fn is_titlecase(&self) -> bool;
}

impl TitleCase for char {
    fn to_titlecase(self) -> ToTitleCase {
        ToTitleCase(CaseMappingIter::new(to_titlecase(self)))
    }

    fn to_titlecase_tr_or_az(self) -> ToTitleCase {
        ToTitleCase(CaseMappingIter::new(to_titlecase_tr_or_az(self)))
    }

    fn is_titlecase(&self) -> bool {
        TITLECASE_TABLE
            .binary_search_by(|&(key, _)| key.cmp(self))
            .is_err()
    }
}


/// Trait to add titlecase operations to Strings and string slices. Both locale agnostic and TR/AZ
/// versions of the functions are supplied.
pub trait StrTitleCase {
    /// Titlecases the first char of a string, leaves the rest unchanged, and returns a copy.
    ///
    /// # Examples
    /// If the str is already titlecase then it will return itself
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("ABC".to_titlecase(), "ABC")
    /// ```
    /// Single-char characters are mapped:
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("ǄǄ".to_titlecase(), "ǅǄ")
    /// ```
    /// Multi-char ligatures are converted:
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("ﬄabc".to_titlecase(), "Fflabc")
    /// ```
    /// Locale is ignored:
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("iii".to_titlecase(), "Iii")
    /// ```
    /// # Locale
    /// This function is not locale specific. Unicode special casing has rules for tr and az that
    /// this function does not take into account. For tr and az locales use [`StrTitleCase::to_titlecase_tr_or_az`]
    fn to_titlecase(&self) -> String;
    /// Titlecases the first char of a string, lowercases the rest of the string, and returns a copy.
    ///
    /// # Examples
    /// If the str is already titlecase then it will return itself
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("ABC".to_titlecase_lower_rest(), "Abc")
    /// ```
    /// Single-char characters are mapped:
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("ǄǄ".to_titlecase_lower_rest(), "ǅǆ")
    /// ```
    /// Multi-char ligatures are converted:
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("ﬄabc".to_titlecase_lower_rest(), "Fflabc")
    /// ```
    /// Locale is ignored:
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("iIi".to_titlecase_lower_rest(), "Iii")
    /// ```
    /// # Locale
    /// This function is not locale specific. Unicode special casing has rules for tr and az that
    /// this function does not take into account. For tr and az locales use [`StrTitleCase::to_titlecase_tr_or_az_lower_rest`]
    fn to_titlecase_lower_rest(&self) -> String;
    /// This functions the same way as [`StrTitleCase::to_titlecase`] except that it uses the TR/AZ
    /// locales. This has one major change:
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("iIi".to_titlecase_tr_or_az(), "İIi")
    /// ```
    ///
    /// For the locale agnostic version use [`StrTitleCase::to_titlecase`].
    fn to_titlecase_tr_or_az(&self) -> String;
    /// This functions the same way as [`StrTitleCase::to_titlecase_lower_rest`] except that it uses
    /// the TR/AZ locales. This has one major change, 'i' maps to 'İ':
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert_eq!("iIiİ".to_titlecase_tr_or_az_lower_rest(), "İıii")
    /// ```
    ///
    /// When lowercasing in these locales there are a couple differences to note:
    ///   1) 'I' maps to 'ı'--\u{0049} LATIN CAPITAL LETTER I -> \u{0131} LATIN SMALL LETTER DOTLESS I
    ///   2) 'İ' maps to 'i'--\u{130} LATIN CAPITAL LETTER I WITH DOT ABOVE -> \u{0069} LATIN SMALL LETTER I
    ///
    /// For the locale agnostic version use [`StrTitleCase::to_titlecase_lower_rest`].
    fn to_titlecase_tr_or_az_lower_rest(&self) -> String;

    /// Tests if the first char of this string is titlecase. This is locale agnostic and returns the
    /// same values in the tr/az locales.
    /// # Returns
    /// True if the first character of the string is title case, ignoring the rest of the string.
    /// False if first character is not title case or the string is empty.
    /// # Examples
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert!("Abc".starts_titlecase());
    /// assert!("ABC".starts_titlecase());
    ///
    /// assert!(!"abc".starts_titlecase());
    /// ```
    fn starts_titlecase(&self) -> bool;

    /// Tests if the first char of this string is titlecase and the rest of the string is lowercase.
    /// This is locale agnostic and returns the same values in the tr/az locales.
    /// # Returns
    /// True if the first character of the string is title case and the rest of the string is lowercase.
    /// False if first character is not title case or the string is empty.
    /// # Examples
    /// ```
    /// use unicode_titlecase::StrTitleCase;
    /// assert!("Abc".starts_titlecase_rest_lower());
    /// assert!("İbc".starts_titlecase_rest_lower());
    ///
    /// assert!(!"abc".starts_titlecase_rest_lower());
    /// assert!(!"ABC".starts_titlecase_rest_lower());
    /// assert!(!"İİ".starts_titlecase_rest_lower());
    /// ```
    fn starts_titlecase_rest_lower(&self) -> bool;
}

impl StrTitleCase for str {
    fn to_titlecase(&self) -> String {
        let mut iter = self.chars();
        iter.next()
            .into_iter()
            .flat_map(TitleCase::to_titlecase)
            .chain(iter)
            .collect()
    }

    fn to_titlecase_lower_rest(&self) -> String {
        let mut iter = self.chars();
        iter.next()
            .into_iter()
            .flat_map(TitleCase::to_titlecase)
            .chain(iter.flat_map(char::to_lowercase))
            .collect()
    }

    fn to_titlecase_tr_or_az(&self) -> String {
        let mut iter = self.chars();
        iter.next()
            .into_iter()
            .flat_map(TitleCase::to_titlecase_tr_or_az)
            .chain(iter)
            .collect()
    }

    fn to_titlecase_tr_or_az_lower_rest(&self) -> String {
        let mut iter = self.chars();
        iter.next()
            .into_iter()
            .flat_map(TitleCase::to_titlecase_tr_or_az)
            .chain(iter.map(to_lowercase_tr_or_az))
            .collect()
    }

    fn starts_titlecase(&self) -> bool {
        self.chars()
            .next()
            .as_ref()
            .map_or(false, TitleCase::is_titlecase)
    }

    fn starts_titlecase_rest_lower(&self) -> bool {
        let mut iter = self.chars();
        iter.next()
            .as_ref()
            .map_or(false, TitleCase::is_titlecase)
            && iter.all(char::is_lowercase)
    }
}

pub mod tr_az {
    use alloc::string::String;
    use core::fmt::{Display, Formatter, Result};
    use core::iter::{FusedIterator, once};

    use crate::CaseMappingIter;

    /// Accepts a char and returns the Unicode Upper Case in the TR/AZ locale for that character as a an iterator.
    ///
    /// # Examples
    /// ```
    /// use unicode_titlecase::tr_az::to_uppercase_tr_or_az;
    /// assert_eq!(to_uppercase_tr_or_az('i').to_string(), "İ");
    /// assert_eq!(to_uppercase_tr_or_az('a').to_string(), "A");
    /// assert_eq!(to_uppercase_tr_or_az('ﬀ').to_string(), "FF");
    /// ```
    #[must_use]
    pub fn to_uppercase_tr_or_az(c: char) -> TrAzCaseMapper {
        TrAzCaseMapper::new(once(c)
            .map(|c| match c {
                '\u{0069}' => '\u{0130}', //i => İ
                _ => c,
            })
            .flat_map(char::to_uppercase))
    }

    /// Accepts a char and returns the Unicode Lower Case in the TR/AZ locale for that character as a an iterator.
    ///
    /// # Examples
    /// ```
    /// use unicode_titlecase::tr_az::to_lowercase_tr_or_az;
    /// assert_eq!(to_lowercase_tr_or_az('İ'), 'i');
    /// assert_eq!(to_lowercase_tr_or_az('I'), 'ı');
    /// assert_eq!(to_lowercase_tr_or_az('A'), 'a');
    /// ```
    /// # Implementation Note
    /// This function is able to return a char instead of an iterator because the TR/AZ locales
    /// don't have any characters that map to multiple lowercase characters. If that ever changes
    /// then this function will have to change to an iterator and have a corresponding bump in the
    /// major version of the crate. A change like that seems unlikely enough to warrant this risk
    /// and optimization.
    #[must_use]
    pub fn to_lowercase_tr_or_az(c: char) -> char {
        match c {
            '\u{0049}' => '\u{0131}', //I => ı
            '\u{0130}' => '\u{0069}', //İ => i
            _ => c.to_lowercase().next().unwrap(), //safe because to_lowercase will at least return c
        }
    }

    pub trait LowerCaseTrAz {
        fn to_lowercase_tr_az(self) -> char;
        fn is_lowercase_tr_az(&self) -> bool;
        fn to_uppercase_tr_az(self) -> TrAzCaseMapper;
        fn is_uppercase_tr_az(&self) -> bool;
    }

    impl LowerCaseTrAz for char {
        fn to_lowercase_tr_az(self) -> char {
            to_lowercase_tr_or_az(self)
        }

        fn is_lowercase_tr_az(&self) -> bool {
            self.is_lowercase()
        }

        fn to_uppercase_tr_az(self) -> TrAzCaseMapper {
            to_uppercase_tr_or_az(self)
        }

        fn is_uppercase_tr_az(&self) -> bool {
            self.is_uppercase()
        }
    }

    pub trait StrLowerCaseTrAz {
        fn to_lowercase_tr_az(&self) -> String;
        fn is_lowercase_tr_az(&self) -> bool;
        fn to_uppercase_tr_az(&self) -> String;
        fn is_uppercase_tr_az(&self) -> bool;
    }

    impl StrLowerCaseTrAz for str {
        fn to_lowercase_tr_az(&self) -> String {
            self.chars().map(to_lowercase_tr_or_az).collect()
        }

        fn is_lowercase_tr_az(&self) -> bool {
            self.chars().all(|c| c.is_lowercase_tr_az())
        }

        fn to_uppercase_tr_az(&self) -> String {
            self.chars().flat_map(to_uppercase_tr_or_az).collect()
        }

        fn is_uppercase_tr_az(&self) -> bool {
            self.chars().all(|c| c.is_uppercase_tr_az())
        }
    }

    /// An iterator over a titlecase mapped char.
    ///
    /// Copied from the std library's [`core::char::ToLowercase`] and [`core::char::ToUppercase`].
    #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub struct TrAzCaseMapper(CaseMappingIter);

    impl TrAzCaseMapper {
        fn new(mut chars: impl Iterator<Item=char>) -> Self {
            TrAzCaseMapper(CaseMappingIter::new([chars.next().unwrap_or('\0'), chars.next().unwrap_or('\0'), chars.next().unwrap_or('\0'), ]))
        }
    }

    impl Iterator for TrAzCaseMapper {
        type Item = char;
        fn next(&mut self) -> Option<char> {
            self.0.next()
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }

    impl DoubleEndedIterator for TrAzCaseMapper {
        fn next_back(&mut self) -> Option<char> {
            self.0.next_back()
        }
    }

    impl FusedIterator for TrAzCaseMapper {}

    impl ExactSizeIterator for TrAzCaseMapper {}

    impl Display for TrAzCaseMapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            core::fmt::Display::fmt(&self.0, f)
        }
    }
}

/// An iterator over a titlecase mapped char.
///
/// Copied from the std library's [`core::char::ToLowercase`] and [`core::char::ToUppercase`].
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

impl Display for ToTitleCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

// Copied out of the std library
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

impl Display for CaseMappingIter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
