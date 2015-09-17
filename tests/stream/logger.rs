#![allow(non_snake_case)]

extern crate io_providers;

use std::io::{Read, Write};
use self::io_providers::StreamProvider;
use self::io_providers::stream::Logger;

#[test]
fn sanity_check() {
    // Logger just wraps other objects, so there's not much to do except for
    // one integration sanity check.
    let mut buf = vec![0; 5];
    let input = vec![1, 2, 3, 4, 5];
    let mut logger = Logger::new(vec![Ok(input.clone())]);

    let result = logger.input().read(&mut buf);
    assert_eq!(5, result.unwrap());
    assert_eq!(input, buf);

    buf = vec![1, 3, 3, 7];
    let result = logger.output().write(&buf[..]);
    assert_eq!(buf.len(), result.unwrap());
    assert_eq!(&buf[..], logger.get_output());

    buf = vec![1, 0, 1];
    let result = logger.error().write(&buf[..]);
    assert_eq!(buf.len(), result.unwrap());
    assert_eq!(&buf[..], logger.get_error());
}
