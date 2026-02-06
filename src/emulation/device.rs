//! Emulation for different browsers.

#[macro_use]
mod macros;
pub mod chrome;

pub use typed_builder::TypedBuilder;
#[cfg(feature = "emulation-compression")]
pub use wreq::header::ACCEPT_ENCODING;
pub use wreq::{
    Emulation,
    header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderName, HeaderValue, USER_AGENT},
    http2::{
        Http2Options, PseudoId, PseudoOrder, SettingId, SettingsOrder, StreamDependency, StreamId,
    },
    tls::{AlpsProtocol, CertificateCompressionAlgorithm, TlsOptions, TlsVersion},
};

pub use crate::emulation::{EmulationOS, EmulationOption};
