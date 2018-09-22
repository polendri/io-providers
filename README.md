# io-providers [![Build Status](https://travis-ci.org/pshendry/io-providers.svg)](https://travis-ci.org/pshendry/io-providers) [![Latest Version](https://img.shields.io/crates/v/io-providers.svg)](https://crates.io/crates/io-providers) [![Documentation](https://docs.rs/io-providers/badge.svg)](https://docs.rs/io-providers) [![License](https://img.shields.io/crates/l/io_providers.svg)](https://github.com/pshendry/io-providers/blob/master/LICENSE)

Defines "provider" traits and implementations for different types of I/O operations, enabling
dependency injection that's very helpful for testing.

A number of different I/O types are supported:

* Process environment (variables, working directy etc), via [`Env`](https://docs.rs/io-providers/latest/io_providers/env/trait.Env.html)
* Standard streams (stdin, stdout and stderr), via [`StdStreams`](https://docs.rs/io-providers/latest/io_providers/std_streams/trait.StdStreams.html)
* Filesystem access, via [`Fs`](https://docs.rs/io-providers/latest/io_providers/fs/trait.Fs.html)

In addition to "native" implementations for each trait, "simulated" implementations are also
built-in:

* [`SimulatedEnv`](https://docs.rs/io-providers/latest/io_providers/env/trait.SimulatedEnv.html) for faking process environment state
* [`SimulatedStdStreams`](https://docs.rs/io-providers/latest/io_providers/std_streams/trait.SimulatedStdStreams.html) for faking standard
  stream input and inspecting output
* [`TempFs`](https://docs.rs/io-providers/latest/io_providers/fs/trait.TempFs.html) for performing filesystem access in a `chroot`-like sandbox
  isolated from the rest of the filesystem

Each provider trait can be used independently, however there is also the all-encompassing
[`Io`](https://docs.rs/io-providers/latest/io_providers/trait.Io.html) which provides access to all of them. If you have a variety of I/O
dependencies, it might be easiest to create and pass around a single `&mut Io`.

## Documentation

https://docs.rs/io-providers/

## Examples

`Cargo.toml`:

```toml
[dependencies]
io-providers = "0.2.0-beta.2"
```

`src/main.rs`:

```rust
extern crate io_providers;

use std::io::Write;
use std::path::Path;
use io_providers::{Env, Io, NativeIo, SimulatedIo, StdStreams};

/// Gets the current working directory and prints it to stdout.
fn do_work<I: Io>(io: &mut I) {
    let cur_dir = io.env().current_dir().unwrap();
    let stdout = io.std_streams().output();
    writeln!(stdout, "The current directory is: {}", cur_dir.to_str().unwrap()).unwrap();
}

fn main() {
    // Test `do_work()` using a simulated I/O environment
    let mut simulated_io = SimulatedIo::new().unwrap();
    simulated_io.env_mut().set_current_dir(Path::new("/foo/bar")).unwrap();
    do_work(&mut simulated_io);
    assert_eq!(
        "The current directory is: /foo/bar\n",
        ::std::str::from_utf8(simulated_io.std_streams().read_output()).unwrap());

    // Now use a native I/O provided to access the real system
    let mut real_io = NativeIo::new();
    do_work(&mut real_io);
}
```

## License

`io-providers` is distributed under the [MIT license](https://opensource.org/licenses/MIT).
