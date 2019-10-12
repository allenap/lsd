use std::process::Command;

pub use assert_cmd::prelude::*;
pub use assert_fs::prelude::*;
pub use predicates::prelude::*;

pub fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

pub fn tempdir() -> assert_fs::TempDir {
    assert_fs::TempDir::new().unwrap()
}
