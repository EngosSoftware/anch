### anch

[![crates.io][crates-badge]][crates-url]
![build][build-badge]
![coverage][cov-badge]
![mbh][mbh-badge]

[crates-badge]: https://img.shields.io/crates/v/anch.svg?refresh=1
[crates-url]: https://crates.io/crates/anch
[cov-badge]: https://img.shields.io/badge/cov-100%25-21b577.svg
[build-badge]: https://github.com/EngosSoftware/anch/actions/workflows/build.yml/badge.svg
[mbh-badge]: https://img.shields.io/badge/Made_by-HUMAN-F5DEB3.svg

# Annotated characters
                      
This crate provides an iterator that yields annotated characters, meaning each item
is either a character or a line ending, together with the line and column where it was
encountered in the input. Lines and columns are numbered starting from 1.

All valid line endings are recognized and reported as a single item, like this:

- `\n` → `LineEnding::Lf`
- `\r` → `LineEnding::Cr`
- `\r\n` → `LineEnding::CrLf`

The original line endings are preserved, so the input can be reconstructed
from the annotated characters when needed.

The annotated iterator can be created by calling the method `annotated`
on any iterator over characters, like in this example:

```rust
use anch::{Annotated, AnnotatedChar, LineEnding};

fn main() {
  let mut chars = "Lorem\r\nipsum".chars().annotated();
  assert_eq!(AnnotatedChar::Character('L', 1, 1), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('o', 1, 2), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('r', 1, 3), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('e', 1, 4), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('m', 1, 5), chars.next().unwrap());
  assert_eq!(AnnotatedChar::LineEnding(LineEnding::CrLf, 1, 6), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('i', 2, 1), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('p', 2, 2), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('s', 2, 3), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('u', 2, 4), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('m', 2, 5), chars.next().unwrap());
  assert_eq!(None, chars.next());
}
```                                                                                                                                                                                      

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
