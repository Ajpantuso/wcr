// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[arg(default_value = "-")]
    pub files: Vec<String>,
    #[arg(short = 'l', long = "lines")]
    pub lines: bool,
    #[arg(short = 'w', long = "words")]
    pub words: bool,
    #[arg(short = 'm', long = "chars")]
    pub chars: bool,
    #[arg(short = 'c', long = "bytes")]
    pub bytes: bool,
}

impl Options {
    pub fn no_selection(&self) -> bool {
        [self.lines, self.words, self.chars, self.bytes]
            .iter()
            .all(|b| !b)
    }
}
