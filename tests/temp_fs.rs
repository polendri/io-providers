#![allow(non_snake_case)]

extern crate io_providers;

use std::env;
use std::path::PathBuf;

use io_providers::fs::{Fs, TempFs};

#[test]
fn fs__uses_system_temp_dir() {
    let fs = TempFs::new().expect("Failed to create new TempFs");

    assert!(fs.path().parent().unwrap() == env::temp_dir());
}

#[test]
fn fs__file_created__exists_in_temp_dir() {
    let mut fs = TempFs::new().expect("Failed to create new TempFs");

    fs.write("test.txt", "contents".as_bytes())
        .expect("Failed to write test file");
    let contents = fs
        .read_to_string("test.txt")
        .expect("Failed to read test file contents");

    assert!(fs.path().join("test.txt").exists());
    assert_eq!("contents", contents);
}

#[test]
fn fs__dropped_from_scope__cleans_up_temp_dir() {
    let temp_dir: PathBuf;

    {
        let fs = TempFs::new().expect("Failed to create new TempFs");
        temp_dir = fs.path().to_path_buf();

        assert!(temp_dir.exists());
    }

    assert!(!temp_dir.exists());
}
