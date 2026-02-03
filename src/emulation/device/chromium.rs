//! Chromium-accurate emulation module.
//!
//! This module provides fingerprint-accurate Chromium emulation profiles
//! that match the exact TLS and HTTP/2 settings from Chromium source code.
//!
//! Unlike the `chrome` module which optimizes for performance (larger window sizes),
//! these profiles prioritize fingerprint accuracy for anti-bot evasion.

#[macro_use]
mod http2;
#[macro_use]
mod tls;
mod header;

use header::*;

use super::*;

mod_generator!(
    chromium_latest,
    tls_options!(),
    http2_options!(),
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="143", "Not:A-Brand";v="24", "Chromium";v="143""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="143", "Not:A-Brand";v="24", "Chromium";v="143""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="143", "Not:A-Brand";v="24", "Chromium";v="143""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="143", "Not:A-Brand";v="24", "Chromium";v="143""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="143", "Not:A-Brand";v="24", "Chromium";v="143""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/143.0.6943.0 Mobile/15E148 Safari/604.1"
        )
    ]
);
