# Unicode Titlecase
Unicode titlecasing operations for chars and strings. The crate supports has additional
functionality to support the TR/AZ locales.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
unicode_titlecase = "1.1.0"
```

## Features

This crate is no_std capable. ```std``` is used for
```std::Display``` on the iterators, allowing the usage of ```to_string()```.

## Usage

### Chars

To turn a ```char``` into its titlecase equivalent ```[char; 3]```array:

```rust
use unicode_titlecase::to_titlecase;

assert_eq!(to_titlecase('A'), ['A', '\0', '\0']);
assert_eq!(to_titlecase('Ǆ'), ['ǅ', '\0', '\0']);
assert_eq!(to_titlecase('ﬄ'), ['F', 'f', 'l']);
```

Or use the iterator version that follows the same format as the std library. The crate defines
a ```Trait``` that is implemented on ```char```:

```rust
use unicode_titlecase::TitleCase;
assert_eq!('i'.to_titlecase().to_string(), "I");
assert_eq!('A'.to_titlecase().to_string(), "A");
assert_eq!('Ǆ'.to_titlecase().to_string(), "ǅ");
assert_eq!('ﬄ'.to_titlecase().to_string(), "Ffl");
```

### Strings

A similar trait is defined on ```str```. This
will titlecase the first char of the string, leave the rest unchanged, and return a newly
allocated ```String```.

```rust
use unicode_titlecase::StrTitleCase;
assert_eq!("iii".to_titlecase(), "Iii");
assert_eq!("ABC".to_titlecase(), "ABC");
assert_eq!("ǄǄ".to_titlecase(), "ǅǄ");
assert_eq!("ﬄabc".to_titlecase(), "Fflabc");
```

Alternatively, you could lowercase the rest of the ```str```:

```rust
use unicode_titlecase::StrTitleCase;
assert_eq!("iIi".to_titlecase_lower_rest(), "Iii");
assert_eq!("ABC".to_titlecase_lower_rest(), "Abc");
assert_eq!("ǄǄ".to_titlecase_lower_rest(), "ǅǆ");
assert_eq!("ﬄabc".to_titlecase_lower_rest(), "Fflabc");
```

### Testing a char or str

To see if the char is already titlecase, ```is_titlecase``` is provided:

```rust
use unicode_titlecase::TitleCase;
assert!('A'.is_titlecase());
assert!('ǅ'.is_titlecase());
assert!('İ'.is_titlecase());

assert!(!'a'.is_titlecase());
assert!(!'Ǆ'.is_titlecase());
assert!(!'ﬄ'.is_titlecase());
```

To test if a str is already titlecase, two options are provided. The first, ```starts_titlecase```
returns true if the first character is titlecased--ignoring the rest of the str. The second
```starts_titlecase_rest_lower``` only returns true if the first char is titlecase and the rest
of the str is lowercase.

```rust
use unicode_titlecase::StrTitleCase;
assert!("Abc".starts_titlecase());
assert!("ABC".starts_titlecase());
assert!(!"abc".starts_titlecase());

assert!("Abc".starts_titlecase_rest_lower());
assert!("İbc".starts_titlecase_rest_lower());
assert!(!"abc".starts_titlecase_rest_lower());
assert!(!"ABC".starts_titlecase_rest_lower());
assert!(!"İİ".starts_titlecase_rest_lower());
```

All testing functions work the same regardless of locale.

### Locale

The TR and AZ locales have different rules for how to titlecase certain characters.
The ```to_titlecase``` functions assume the locale is neither of these locations. A "tr_or_az"
version of each function is provided instead.

```rust
use unicode_titlecase::{to_titlecase_tr_or_az, StrTitleCase};
assert_eq!(to_titlecase_tr_or_az('i'), ['İ', '\0', '\0']);
assert_eq!('i'.to_titlecase_tr_or_az().to_string(), "İ");
assert_eq!("iIi".to_titlecase_tr_or_az(), "İIi");
assert_eq!("iIi".to_titlecase_tr_or_az_lower_rest(), "İii");
```

## License

`unicode_titlecase` is licensed under the [MIT License](LICENSE.txt)