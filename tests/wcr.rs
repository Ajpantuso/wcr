// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod wcr {
    use anyhow::Result;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use std::path;
    use test_case::test_case;

    #[test_case("", &["ascii.txt"], load_output("ascii_no_options.txt") ; "no blank input/no options")]
    fn valid(stdin: &str, args: &[&str], output: String) -> Result<()> {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .write_stdin(stdin)
            .args(args)
            .current_dir(root().join("inputs"))
            .assert()
            .stdout(predicate::str::contains(output))
            .success();

        Ok(())
    }
    fn load_output(name: &str) -> String {
        fs::read_to_string(root().join("outputs").join(name)).unwrap()
    }
    fn root<'a>() -> &'a path::Path {
        path::Path::new("tests")
    }
}
