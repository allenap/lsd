mod common;
use common::*;

#[test]
fn test_list_file_in_subdirectory() {
    let dir = tempdir();
    let subdir = dir.child("subdir");
    subdir.create_dir_all().unwrap();
    let child = subdir.child("child");
    child.touch().unwrap();
    cmd()
        .current_dir(dir.path())
        .arg("subdir/child")
        .assert()
        // GNU and BSD `ls` includes the path given at the command-line in the
        // rendered listing.
        .stdout(predicate::eq("subdir/child\n"));
}

#[test]
fn test_list_files_in_subdirectory() {
    let dir = tempdir();
    let subdir = dir.child("subdir");
    subdir.create_dir_all().unwrap();
    let child1 = subdir.child("child1");
    child1.touch().unwrap();
    let child2 = subdir.child("child2");
    child2.touch().unwrap();
    cmd()
        .current_dir(dir.path())
        .arg("subdir/child1")
        .arg("subdir/child1")
        .arg("subdir/child2")
        .arg("./subdir/child2")
        .assert()
        // GNU and BSD `ls` includes all the paths given at the command-line in
        // the rendered listing, even if they're duplicates, or different ways
        // to refer to the same file. Curiously it appears that BSD sorts the
        // result by the whole rendered path whereas GNU sorts by basename first
        // then by whole path. Below is the GNU order.
        .stdout(predicate::eq(concat!(
            "subdir/child1\n",
            "subdir/child1\n",
            "./subdir/child2\n",
            "subdir/child2\n",
        )));
}
