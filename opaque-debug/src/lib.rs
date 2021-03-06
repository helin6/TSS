//! Macro for opaque `Debug` trait implementation.

#![no_std]
#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)] #![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]



#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/opaque-debug/0.3.0"
)]

#[doc(hidden)]
pub extern crate core as __core;

/// Macro for defining opaque `Debug` implementation.
///
/// It will use the following format: "StructName { ... }". While it's
/// convinient to have it (e.g. for including into other structs), it could be
/// undesirable to leak internal state, which can happen for example through
/// uncareful logging.
#[macro_export]
macro_rules! implement {
    ($struct:ty) => {
        impl $crate::__core::fmt::Debug for $struct {
            fn fmt(
                &self,
                f: &mut $crate::__core::fmt::Formatter,
            ) -> Result<(), $crate::__core::fmt::Error> {
                write!(f, concat!(stringify!($struct), " {{ ... }}"))
            }
        }
    };
}


#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))] #[macro_use] extern crate sgx_tstd as std;