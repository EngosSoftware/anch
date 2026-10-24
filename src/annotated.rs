//! # Annotated iterator implementation

use crate::line_ending::LineEnding;

/// Enumeration of annotated characters.
///
/// The character can be any Unicode character (excluding line endings)
/// or a line ending itself. Additionally, each enumeration variant stores
/// the line and column position where the character or line ending
/// was encountered in the source text.
/// Lines and columns are 1-based.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AnnotatedChar {
    /// Any character except line ending.
    ///
    /// This enumeration variant holds the character, line and column.
    Character(char, usize, usize),
    /// Line ending.
    ///
    /// This enumeration variant holds the line ending, line and column.
    LineEnding(LineEnding, usize, usize),
}

impl AnnotatedChar {
    /// Returns the position (line, column) of the annotated character in the input.
    #[must_use]
    pub fn position(self) -> (usize, usize) {
        match self {
            AnnotatedChar::Character(_, line, column)
            | AnnotatedChar::LineEnding(_, line, column) => (line, column),
        }
    }
}

/// Trait extension for iterating over annotated characters.
pub trait Annotated: Iterator<Item = char> {
    /// Returns an iterator over annotated characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use anch::{Annotated, AnnotatedChar, LineEnding};
    ///
    /// let mut chars = "Lorem\r\nipsum".chars().annotated();
    /// assert_eq!(AnnotatedChar::Character('L', 1, 1), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('o', 1, 2), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('r', 1, 3), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('e', 1, 4), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('m', 1, 5), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::LineEnding(LineEnding::CrLf, 1, 6), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('i', 2, 1), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('p', 2, 2), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('s', 2, 3), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('u', 2, 4), chars.next().unwrap());
    /// assert_eq!(AnnotatedChar::Character('m', 2, 5), chars.next().unwrap());
    /// assert_eq!(None, chars.next());
    /// ```
    fn annotated(self) -> impl Iterator<Item = AnnotatedChar>
    where
        Self: Sized,
    {
        Annotator {
            iter: self,
            next: None,
            line: 1,
            column: 0,
        }
    }
}

struct Annotator<I> {
    iter: I,
    next: Option<char>,
    line: usize,
    column: usize,
}

impl<I: Iterator<Item = char>> Iterator for Annotator<I> {
    type Item = AnnotatedChar;

    fn next(&mut self) -> Option<AnnotatedChar> {
        let line = self.line;
        let column = self.column + 1;
        match self.next.take().or_else(|| self.iter.next()) {
            Some('\n') => {
                self.line += 1;
                self.column = 0;
                Some(AnnotatedChar::LineEnding(LineEnding::Lf, line, column))
            }
            Some('\r') => {
                self.line += 1;
                self.column = 0;
                if let Some(ch) = self.iter.next() {
                    if ch == '\n' {
                        self.next = None;
                        Some(AnnotatedChar::LineEnding(LineEnding::CrLf, line, column))
                    } else {
                        self.next = Some(ch);
                        Some(AnnotatedChar::LineEnding(LineEnding::Cr, line, column))
                    }
                } else {
                    self.next = None;
                    Some(AnnotatedChar::LineEnding(LineEnding::Cr, line, column))
                }
            }
            other => {
                self.column = column;
                other.map(|ch| AnnotatedChar::Character(ch, line, column))
            }
        }
    }
}

/// Implement annotated for iterator over characters.
impl<I: Iterator<Item = char>> Annotated for I {}
