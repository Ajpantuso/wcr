// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use std::io::{self, Write};

mod input;
pub mod options;
mod stats;

pub struct Command<'a> {
    options: &'a options::Options,
}

impl<'a> Command<'a> {
    pub fn run(&self) -> Result<()> {
        let mut stdout = io::stdout().lock();
        let mut stderr = io::stderr().lock();

        let mut total = stats::FileStatistics::new("total");

        for file in self.options.files.iter() {
            match input::open(file) {
                Ok(input) => {
                    let mut stats = stats::FileStatistics::new(file);

                    match input.to_lines() {
                        Ok(lines) => lines.flatten().for_each(|l| stats.count_line(&l)),
                        Err(e) => {
                            writeln!(stderr, "{}: {}: {}", clap::crate_name!(), input.name(), e)?
                        }
                    }

                    writeln!(
                        stdout,
                        "{}",
                        stats.display_from_options(self.options).unwrap()
                    )?;

                    total += stats;
                }
                Err(e) => writeln!(
                    &mut stderr,
                    "{}: cannot open '{}' for reading: {}",
                    clap::crate_name!(),
                    file,
                    e
                )?,
            }
        }

        if self.options.files.len() > 1 {
            writeln!(
                stdout,
                "{}",
                total.display_from_options(self.options).unwrap()
            )?;
        }

        Ok(())
    }
}

impl<'a> From<&'a options::Options> for Command<'a> {
    fn from(options: &'a options::Options) -> Self {
        Command { options }
    }
}
