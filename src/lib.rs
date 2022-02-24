//! # hyper-tls
//!
//! An HTTPS connector to be used with [hyper][].
//!
//! [hyper]: https://hyper.rs
//!
//! ## Example
//!
//! ```no_run
//! use hyper_tls::HttpsConnector;
//! use hyper::Client;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Box<dyn std::error::Error>>{
//!     let https = HttpsConnector::new();
//!     let client = Client::builder().build::<_, hyper::Body>(https);
//!
//!     let res = client.get("https://hyper.rs".parse()?).await?;
//!     assert_eq!(res.status(), 200);
//!     Ok(())
//! }
//! ```
#![doc(html_root_url = "https://docs.rs/hyper-tls/0.5.0")]
#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[doc(hidden)]
#[cfg(feature = "use-native-tls")]
pub extern crate native_tls;

#[cfg(feature = "use-native-tls")]
pub use tokio_native_tls::{TlsConnector, TlsStream};

#[cfg(all(feature = "use-rustls", not(feature = "use-native-tls")))]
pub use tokio_rustls::{TlsConnector, TlsStream};

#[cfg(any(feature = "use-native-tls", feature = "use-rustls"))]
pub use client::{HttpsConnecting, HttpsConnector};
#[cfg(any(feature = "use-native-tls", feature = "use-rustls"))]
pub use stream::MaybeHttpsStream;

#[cfg(any(feature = "use-native-tls", feature = "use-rustls"))]
mod client;
#[cfg(any(feature = "use-native-tls", feature = "use-rustls"))]
mod stream;
