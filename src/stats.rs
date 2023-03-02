// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use crate::options;
use anyhow::Result;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct FileStatistics {
    name: String,
    line_count: u64,
    word_count: u64,
    char_count: u64,
    byte_count: u64,
}

impl FileStatistics {
    pub fn new(name: &str) -> Self {
        FileStatistics {
            name: String::from(name),
            line_count: 0,
            word_count: 0,
            char_count: 0,
            byte_count: 0,
        }
    }
    pub fn count_line(&mut self, line: &str) {
        self.line_count += 1;
        self.word_count += u64::try_from(line.split_whitespace().count()).unwrap();
        self.char_count += u64::try_from(line.chars().count()).unwrap();
        self.byte_count += u64::try_from(line.len()).unwrap();
    }
    pub fn display_from_options(&self, options: &options::Options) -> Result<String> {
        let mut s = String::new();
        let width = self.min_width(options);

        if options.lines || options.no_selection() {
            write!(s, "{:width$} ", self.line_count)?;
        }
        if options.words || options.no_selection() {
            write!(s, "{:width$} ", self.word_count)?;
        }
        if options.chars {
            write!(s, "{:width$} ", self.char_count)?;
        }
        if options.bytes || options.no_selection() {
            write!(s, "{:width$} ", self.byte_count)?;
        }

        write!(s, "{}", self.name)?;

        Ok(s)
    }
    fn min_width(&self, options: &options::Options) -> usize {
        let mut widths = vec![];

        if options.lines || options.no_selection() {
            widths.push(digits(self.line_count));
        }
        if options.words || options.no_selection() {
            widths.push(digits(self.word_count));
        }
        if options.chars {
            widths.push(digits(self.char_count));
        }
        if options.bytes || options.no_selection() {
            widths.push(digits(self.byte_count));
        }

        *widths.iter().max().unwrap()
    }
}

fn digits(num: u64) -> usize {
    let mut count = 0;

    let mut n = num;
    while n > 0 {
        n /= 10;

        count += 1;
    }

    count
}

impl std::ops::Add for FileStatistics {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        FileStatistics {
            name: self.name,
            line_count: self.line_count + rhs.line_count,
            word_count: self.word_count + rhs.word_count,
            char_count: self.char_count + rhs.char_count,
            byte_count: self.byte_count + rhs.byte_count,
        }
    }
}

impl std::ops::AddAssign for FileStatistics {
    fn add_assign(&mut self, rhs: Self) {
        self.line_count += rhs.line_count;
        self.word_count += rhs.word_count;
        self.char_count += rhs.char_count;
        self.byte_count += rhs.byte_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["abcd efgh ijkl"], 3, 14, 14 ; "one line/three words")]
    #[test_case(&["abcd efgh ijkl", "mnop qrst uvwx"], 6, 28, 28 ; "two lines/three words each")]
    fn count_line(lines: &[&str], words: u64, chars: u64, bytes: u64) {
        let mut stats = FileStatistics::new("test");

        lines.iter().for_each(|l| stats.count_line(l));

        assert_eq!(u64::try_from(lines.len()).unwrap(), stats.line_count);
        assert_eq!(words, stats.word_count);
        assert_eq!(chars, stats.char_count);
        assert_eq!(bytes, stats.byte_count);
    }
}
