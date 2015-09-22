//! Defines "provider" traits and implementations for different types of I/O operations.
//!
//! The purpose of this is mainly for dependency injection: by having your code depend on a
//! generic provider, it can be tested by giving it a virtual, inspectable implementation of that
//! provider. In production, the "real" implementation can be used.
//!
//! Each type of provider exists in its own submodule and can be used independently. However,
//! this module also contains the all-encompassing `IoProvider` trait which provides access to
//! all of them. If you have a lot of I/O dependencies, it might be easier to create and pass
//! around one `&IoProvider` rather than several different providers.
//!
//! TODO: example

pub mod env;
pub mod stream;

/// Provides access to an environment provider and a stream provider.
pub trait IoProvider {
    /// Gets the `env::Provider`.
    fn env<'a>(&'a self) -> &'a env::Provider;

    /// Gets the `stream::Provider`.
    fn stream<'a>(&'a self) -> &'a stream::Provider;
}

/// "Real" implementer of `IoProvider`, using standard streams and the local environment.
pub struct LocalIoProvider {
    env_provider: env::Local,
    stream_provider: stream::Std,
}

impl LocalIoProvider {
    /// Creates a new `LocalIoProvider`.
    pub fn new() -> LocalIoProvider {
        LocalIoProvider {
            env_provider: env::Local,
            stream_provider: stream::Std::new(),
        }
    }
}

impl IoProvider for LocalIoProvider {
    fn env<'a>(&'a self) -> &'a env::Provider {
        &self.env_provider
    }

    fn stream<'a>(&'a self) -> &'a stream::Provider {
        &self.stream_provider
    }
}

/// Virtual implementer of `IoProvider`, using in-memory data which can be inspected.
pub struct VirtualIoProvider {
    env_provider: env::Virtual,
    stream_provider: stream::Virtual,
}

impl VirtualIoProvider {
    /// Creates a new `VirtualIoProvider`.
    pub fn new() -> VirtualIoProvider {
        VirtualIoProvider {
            env_provider: env::Virtual::new(),
            stream_provider: stream::Virtual::new(),
        }
    }
}

impl IoProvider for VirtualIoProvider {
    fn env<'a>(&'a self) -> &'a env::Provider {
        &self.env_provider
    }

    fn stream<'a>(&'a self) -> &'a stream::Provider {
        &self.stream_provider
    }
}
