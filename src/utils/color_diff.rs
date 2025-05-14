use console::Style;
use lazy_static::lazy_static;
use std::fmt;

static LEFT: &str = "<";
static RIGHT: &str = ">";

lazy_static! {
    static ref RED: Style = Style::new().red();
    static ref GREEN: Style = Style::new().green();
    static ref ON_RED: Style = Style::new().red().on_color256(52);
    static ref ON_GREEN: Style = Style::new().green().on_color256(22);
}

pub struct ColorDiff<'a> {
    expected: &'a str,
    actual: &'a str,
}

impl<'a> ColorDiff<'a> {
    pub fn new(expected: &'a str, actual: &'a str) -> Self {
        ColorDiff { expected, actual }
    }
}

impl fmt::Display for ColorDiff<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_lines(f, self.expected, self.actual)
    }
}

// Pretty much taken from "pretty_assertions" just using console instead of ansi_term
macro_rules! paint {
    ($f:expr, $colour:expr, $fmt:expr, $($args:tt)*) => (
        write!($f, "{}", $colour.apply_to(format!($fmt, $($args)*)))
    )
}

/// Delay formatting this deleted chunk until later.
///
/// It can be formatted as a whole chunk by calling `flush`, or the inner value
/// obtained with `take` for further processing (such as an inline diff).
#[derive(Default)]
struct LatentDeletion<'a> {
    // The most recent deleted line we've seen
    value: Option<&'a str>,
    // The number of deleted lines we've seen, including the current value
    count: usize,
}

impl<'a> LatentDeletion<'a> {
    /// Set the chunk value.
    fn set(&mut self, value: &'a str) {
        self.value = Some(value);
        self.count += 1;
    }

    /// Take the underlying chunk value, if it's suitable for inline diffing.
    ///
    /// If there is no value or we've seen more than one line, return `None`.
    fn take(&mut self) -> Option<&'a str> {
        if self.count == 1 {
            self.value.take()
        } else {
            None
        }
    }

    /// If a value is set, print it as a whole chunk, using the given formatter.
    ///
    /// If a value is not set, reset the count to zero (as we've called `flush` twice,
    /// without seeing another deletion. Therefore the line in the middle was something else).
    fn flush<TWrite: fmt::Write>(&mut self, f: &mut TWrite) -> fmt::Result {
        if let Some(value) = self.value {
            paint!(f, RED, "{}{}", LEFT, value)?;
            writeln!(f)?;
            self.value = None;
        } else {
            self.count = 0;
        }

        Ok(())
    }
}

/// Present the diff output for two mutliline strings in a pretty, colorised manner.
pub(crate) fn write_lines<TWrite: fmt::Write>(
    f: &mut TWrite,
    left: &str,
    right: &str,
) -> fmt::Result {
    let diff = ::diff::lines(left, right);

    let mut changes = diff.into_iter().peekable();
    let mut previous_deletion = LatentDeletion::default();

    while let Some(change) = changes.next() {
        match (change, changes.peek()) {
            // If the text is unchanged, just print it plain
            (::diff::Result::Both(value, _), _) => {
                previous_deletion.flush(f)?;
                writeln!(f, " {}", value)?;
            }
            // Defer any deletions to next loop
            (::diff::Result::Left(deleted), _) => {
                previous_deletion.flush(f)?;
                previous_deletion.set(deleted);
            }
            // If we're being followed by more insertions, don't inline diff
            (::diff::Result::Right(inserted), Some(::diff::Result::Right(_))) => {
                previous_deletion.flush(f)?;
                paint!(f, GREEN, "{}{}", RIGHT, inserted)?;
                writeln!(f)?;
            }
            // Otherwise, check if we need to inline diff with the previous line (if it was a deletion)
            (::diff::Result::Right(inserted), _) => {
                if let Some(deleted) = previous_deletion.take() {
                    write_inline_diff(f, deleted, inserted)?;
                } else {
                    previous_deletion.flush(f)?;
                    paint!(f, GREEN, "{}{}", RIGHT, inserted)?;
                    writeln!(f)?;
                }
            }
        };
    }

    previous_deletion.flush(f)?;
    Ok(())
}

/// Format a single line to show an inline diff of the two strings given.
///
/// The given strings should not have a trailing newline.
///
/// The output of this function will be two lines, each with a trailing newline.
fn write_inline_diff<TWrite: fmt::Write>(f: &mut TWrite, left: &str, right: &str) -> fmt::Result {
    let diff = ::diff::chars(left, right);

    paint!(f, RED, "{}", LEFT)?;
    for change in diff.iter() {
        match change {
            ::diff::Result::Both(value, _) => paint!(f, RED, "{}", value)?,
            ::diff::Result::Left(value) => paint!(f, ON_RED, "{}", value)?,
            _ => (),
        }
    }
    writeln!(f)?;

    paint!(f, GREEN, "{}", RIGHT)?;
    for change in diff.iter() {
        match change {
            ::diff::Result::Both(value, _) => paint!(f, GREEN, "{}", value)?,
            ::diff::Result::Right(value) => paint!(f, ON_GREEN, "{}", value)?,
            _ => (),
        }
    }

    writeln!(f)
}
