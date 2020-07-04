// Copyright 2020 Andreas Waidler
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::process::Command;

fn main()
{
    cargo(&["+nightly", "--locked", "fmt", "--all", "--", "--check"]);

    cargo(&[
        "+nightly",
        "--locked",
        "clippy",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--",
        "-Dwarnings",
    ]);

    cargo(&["--locked", "build", "--workspace"]);
    cargo(&["--locked", "build", "--workspace", "--release"]);

    cargo(&["--locked", "test", "--workspace"]);
    cargo(&["--locked", "test", "--workspace", "--release"]);

    // cargo(&["+nightly", "--locked", "miri", "test", "--workspace"]);
    // cargo(&["+nightly", "--locked", "miri", "test", "--workspace",
    // "--release"]);
}

fn cargo<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: std::convert::AsRef<std::ffi::OsStr>,
{
    let status = Command::new("cargo")
        .args(args)
        .status()
        .expect("Failed to execute cargo.");

    match status.code() {
        Some(0) => (),
        Some(e) => std::process::exit(e),
        None => std::process::exit(1),
    }
}
