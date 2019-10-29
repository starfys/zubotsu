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

use git2::{Object, Oid, Repository};
use std::env;
use std::io;
use std::str::FromStr;

/// Accesses the current repository and gets the currently revision string
fn get_git_hash() -> Result<Oid, Box<dyn std::error::Error>> {
    // Get current dir (crate root)
    let cwd = env::current_dir()?;
    // Open the git repo
    let repo = Repository::open(cwd)?;
    // Get the rev spec
    let rev_spec = repo.revparse("HEAD")?;
    // Get the string from the rev spec
    let rev = rev_spec
        .from()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get revision from spec"))?
        .id();
    Ok(rev)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a handle to the local git repository. If this fails, that's okay.
    let git_hash = get_git_hash().map(|oid| oid.to_string()).unwrap_or_else(|_| String::from("UNKNOWN"));
    // Output the git hash to the environment
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    Ok(())
}
