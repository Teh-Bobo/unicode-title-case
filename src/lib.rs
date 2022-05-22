#![no_std]
use core::fmt::Write;
use core::iter::FusedIterator;

include!(concat!(env!("OUT_DIR"), "/casing.rs"));

/// Accepts a char and returns the Unicode Title Case for that character. If there
pub fn to_title_case(c: char) -> [char; 3] {
    if let Ok(index) = TITLECASE_TABLE.binary_search_by(|&(key, _)| key.cmp(&c)) {
        TITLECASE_TABLE[index].1
    } else {
        [c, '\0', '\0']
    }
}

pub trait TitleCase {
    fn to_title_case(self) -> ToTitleCase;
}

impl TitleCase for char {
    fn to_title_case(self) -> ToTitleCase {
        ToTitleCase(CaseMappingIter::new(to_title_case(self)))
    }
}

// Adopted from std library
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

impl core::fmt::Display for CaseMappingIter {
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
