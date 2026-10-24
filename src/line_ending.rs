//! # Line endings

/// Enumeration of line endings.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LineEnding {
    /// Line ending is a line feed character (U+000A).
    Lf,
    /// Line ending is a carriage return character (U+000D).
    Cr,
    /// Line ending is a sequence of carriage return and line feed characters (U+000D U+000A).
    CrLf,
}

impl AsRef<str> for LineEnding {
    fn as_ref(&self) -> &str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::Cr => "\r",
            LineEnding::CrLf => "\r\n",
        }
    }
}

impl LineEnding {
    /// Returns the first character of the line ending.
    ///
    /// # Examples
    ///
    /// ```
    /// use anch::LineEnding;
    ///
    /// assert_eq!('\n', LineEnding::Lf.first());
    /// assert_eq!('\r', LineEnding::Cr.first());
    /// assert_eq!('\r', LineEnding::CrLf.first());
    /// ```
    #[must_use]
    pub const fn first(self) -> char {
        match self {
            LineEnding::Lf => '\n',
            LineEnding::Cr | LineEnding::CrLf => '\r',
        }
    }

    /// Returns the last character of the line ending.
    ///
    /// # Examples
    ///
    /// ```
    /// use anch::LineEnding;
    ///
    /// assert_eq!('\n', LineEnding::Lf.last());
    /// assert_eq!('\r', LineEnding::Cr.last());
    /// assert_eq!('\n', LineEnding::CrLf.last());
    /// ```
    #[must_use]
    pub const fn last(self) -> char {
        match self {
            LineEnding::Lf | LineEnding::CrLf => '\n',
            LineEnding::Cr => '\r',
        }
    }
}
