#![forbid(unsafe_code)]
#![doc = "Rust SDK for `LazyCat` application and device APIs."]

#[allow(clippy::all, clippy::pedantic, non_camel_case_types)]
mod generated {
    include!("gen/mod.rs");
}

pub mod proto {
    pub use crate::generated::cloud::lazycat::apis::{common, localdevice, sys};
    pub use crate::generated::io::containerd;
    pub use crate::generated::lzc::dlna;
}
