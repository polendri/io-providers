# io-providers [![Build Status](https://travis-ci.org/pshendry/io-providers.svg)](https://travis-ci.org/pshendry/io-providers)

Defines "provider" traits and implementations for different types of I/O operations.

* Documentation: https://pshendry.github.io/io-providers/io_providers/
* Crate information: https://crates.io/crates/io-providers

The purpose of this is mainly for dependency injection: by having your code depend on a
generic provider, it can be tested by giving it a virtual, inspectable implementation of that
provider. In production, the "real" implementation can be used.

## Example

`Cargo.toml`:

```
[dependencies]
io-providers = "0.1"
```

`src/main.rs`

```rust
extern crate io_providers;

use std::io::Write;
use std::path::Path;
use io_providers::{IoProvider, LocalIoProvider, VirtualIoProvider};
use io_providers::env::Provider as EnvProvider;
use io_providers::stream::Provider as StreamProvider;

/// Gets the current working directory and prints it to stdout.
fn do_work(io: &mut IoProvider) {
    let cur_dir = io.env().current_dir().unwrap();
    let stdout = io.stream().output();
    writeln!(
        stdout,
        "The current directory is: {}",
        cur_dir.to_str().unwrap())
        .unwrap();
}

fn main() {
    test_do_work_prints_current_dir();

    // Use a local I/O provider here to get real interaction with the system
    let mut io = LocalIoProvider::new();
    do_work(&mut io);
}

fn test_do_work_prints_current_dir() {
    // Use a virtual I/O provider here so we can verify how it was used
    let mut virtual_io = VirtualIoProvider::new();
    virtual_io.env().set_current_dir(Path::new("/foo/bar")).unwrap();

    do_work(&mut virtual_io);

    assert_eq!(
        "The current directory is: /foo/bar\n",
        ::std::str::from_utf8(virtual_io.virtual_stream().read_output()).unwrap());
}
```

## License

`io-providers` is distributed under the [MIT license](https://opensource.org/licenses/MIT).
