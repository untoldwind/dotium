use console::Style;
use dissimilar::{diff, Chunk};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::fmt;

static LEFT: &str = "<";
static NL_LEFT: &str = "\n<";
static RIGHT: &str = ">";
static NL_RIGHT: &str = "\n>";

lazy_static! {
    static ref red: Style = Style::new().red();
    static ref green: Style = Style::new().green();
    static ref on_red: Style = Style::new().on_red().white();
    static ref on_green: Style = Style::new().on_green().white();
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

impl<'a> fmt::Display for ColorDiff<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        color_diff(f, &self.expected, &self.actual)
    }
}

pub fn color_diff(f: &mut fmt::Formatter, expected: &str, actual: &str) -> fmt::Result {
    let changeset = diff(expected, actual);
    fmt_changeset(f, &changeset)
}

fn fmt_changeset(f: &mut fmt::Formatter, changeset: &[Chunk]) -> fmt::Result {
    writeln!(
        f,
        "{} {} / {} {}",
        red.apply_to(LEFT),
        red.apply_to("left"),
        green.apply_to(RIGHT),
        green.apply_to("right")
    )?;

    for (i, diff) in changeset.iter().enumerate() {
        match diff {
            Chunk::Equal(text) => fmt_same(f, text)?,
            Chunk::Insert(added) => {
                if let Some(Chunk::Delete(removed)) = i.checked_sub(1).map(|i| &changeset[i]) {
                    fmt_add_rem(f, added, removed)?;
                } else {
                    fmt_add(f, added)?;
                }
            }
            Chunk::Delete(removed) => {
                if let Some(Chunk::Insert(_)) = changeset.get(i + 1) {
                    continue;
                } else {
                    fmt_rem(f, removed)?;
                }
            }
        }
    }

    Ok(())
}

fn fmt_add_rem(f: &mut fmt::Formatter, added: &str, removed: &str) -> fmt::Result {
    let diffs = dissimilar::diff(removed, added);

    write!(f, "{}", red.apply_to(LEFT))?;
    for diff in &diffs {
        match diff {
            Chunk::Equal(text) => {
                for blob in Itertools::intersperse(text.split('\n'), NL_LEFT) {
                    write!(f, "{}", red.apply_to(blob))?;
                }
            }
            Chunk::Delete(text) => {
                for blob in Itertools::intersperse(text.split('\n'), NL_LEFT) {
                    write!(f, "{}", on_red.apply_to(blob))?;
                }
            }
            Chunk::Insert(_) => continue,
        }
    }
    writeln!(f)?;

    write!(f, "{}", green.apply_to(RIGHT))?;
    for diff in &diffs {
        match diff {
            Chunk::Equal(text) => {
                for blob in Itertools::intersperse(text.split('\n'), NL_RIGHT) {
                    write!(f, "{}", green.apply_to(blob))?;
                }
            }
            Chunk::Insert(text) => {
                for blob in Itertools::intersperse(text.split('\n'), NL_RIGHT) {
                    write!(f, "{}", on_green.apply_to(blob))?;
                }
            }
            Chunk::Delete(_) => continue,
        }
    }
    writeln!(f)?;

    Ok(())
}

fn fmt_add(f: &mut fmt::Formatter, text: &str) -> fmt::Result {
    for line in text.split('\n') {
        writeln!(f, "{}{}", green.apply_to(RIGHT), green.apply_to(line))?;
    }
    Ok(())
}

fn fmt_rem(f: &mut fmt::Formatter, text: &str) -> fmt::Result {
    for line in text.split('\n') {
        writeln!(f, "{}{}", red.apply_to(LEFT), red.apply_to(line))?;
    }
    Ok(())
}

fn fmt_same(f: &mut fmt::Formatter, text: &str) -> fmt::Result {
    for line in text.split('\n') {
        writeln!(f, "{}", line)?;
    }
    Ok(())
}
