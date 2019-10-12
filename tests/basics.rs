mod common;
use common::*;

#[test]
fn test_runs_okay() {
    cmd().assert().success();
}

#[test]
fn test_list_empty_directory() {
    cmd()
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::eq(""));
}

#[test]
fn test_list_all_empty_directory() {
    cmd()
        .arg("--all")
        .arg(tempdir().path())
        .assert()
        .stdout(predicate::eq(".\n..\n"));
}

#[test]
fn test_list_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();
    cmd()
        .arg(dir.path())
        .assert()
        .stdout(predicate::eq("one\ntwo\n"));
}

#[test]
fn test_list_all_populated_directory() {
    let dir = tempdir();
    dir.child("one").touch().unwrap();
    dir.child("two").touch().unwrap();
    cmd()
        .arg("--all")
        .arg(dir.path())
        .assert()
        .stdout(predicate::eq(".\n..\none\ntwo\n"));
}
