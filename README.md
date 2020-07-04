`foundation-rs`: putting Rust projects on a solid base
======================================================

`foundation-rs` is a set of repositories that contain
common files aimed to put (new) Rust projects
on a solid base as fast as possible.
These files include:

  * a proper `rustfmt` configuration file with a license header,
  * a cargo workspace for auxiliary build tools,
  * the build tool `quality-control` which runs
    `rustfmt --check`, `clippy`, debug/release tests, and optionally `miri`,
    in a platform-independent manner,
  * the cargo alias `cargo quality-control`,
  * a GitHub Action that runs `cargo quality-control`
    on the new `HEAD` after a `git push`
  * a GitHub Action that runs `cargo quality-control`
    on every single commit in a pull request.

The repository `foundation-common-rs` contains all files that
library and binary Rust crates can have in common.
The repository `foundation-lib-rs`
contains the common files as well as
an otherwise empty Rust project created by `cargo new --lib`.
The repository `foundation-bin-rs`
contains the common files as well as
an otherwise empty Rust project created by `cargo new --bin`.


Disclaimer
----------

`foundation-rs` is a setup that I deem to be useful for Rust projects,
especially for my own Rust projects.
YMMV.


Thank You
---------

The implementation of the `tools` workspace and `cargo quality-control`
is based on [the blog post “Make Your Own Make” by matklad][MYOM].

[MYOM]: https://matklad.github.io/2018/01/03/make-your-own-make.html
