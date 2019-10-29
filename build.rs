// Copyright 2019 Steven Sheffey
// This file is part of Zubotsu.

// Zubotsu is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Zubotsu is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Zubotsu.  If not, see <https://www.gnu.org/licenses/>.

use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the git revision by running git
    // TODO: use a git crate
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output()?.stdout;
    // Convert the output to a string
    let git_hash = String::from_utf8(output)?;
    // Output the git hash to the environment
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    Ok(())
}
