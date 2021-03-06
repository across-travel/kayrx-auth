//! HTTP Authorization support for kyrx framework.
//!
//! Provides:
//!  * typed [Authorization] and  [WWW-Authenticate] headers
//!  * [extractors] for an [Authorization] header
//!  * [middleware] for easier authorization checking
//!
//! ## Supported schemes
//!
//!  * `Basic`, as defined in [RFC7617](https://tools.ietf.org/html/rfc7617)
//!  * `Bearer`, as defined in [RFC6750](https://tools.ietf.org/html/rfc6750)
//!
//! [Authorization]: `crate::headers::authorization::Authorization`
//! [WWW-Authenticate]: `crate::headers::www_authenticate::WwwAuthenticate`
//! [extractors]: https://kayrx.xyz/kayrx#extractor/
//! [middleware]: ./middleware/

#![deny(bare_trait_objects)]
#![deny(missing_docs)]
#![deny(nonstandard_style)]
#![deny(rust_2018_idioms)]
#![deny(unused)]
#![deny(clippy::all)]

pub mod extractors;
pub mod headers;
pub mod middleware;
mod utils;
