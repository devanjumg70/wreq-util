//! Chromium-accurate TLS configuration.
//!
//! These settings match the exact TLS configuration from Chromium source code,
//! prioritizing fingerprint accuracy over performance optimization.

use super::*;

/// Chromium's cipher list using BoringSSL mini-language.
/// Source: net/ssl/ssl_config.cc
pub const CIPHER_LIST: &str = "ALL:!aPSK:!ECDSA+SHA1:!3DES";

/// Chromium's named groups (curves) for key exchange.
/// Source: net/ssl/ssl_config.cc - Post-quantum hybrid enabled (2024+)
pub const CURVES: &str = join!(":", "X25519MLKEM768", "X25519", "P-256", "P-384");

/// Chromium's signature algorithms.
/// SHA-1 based signatures are not included (disabled in Chromium).
pub const SIGALGS_LIST: &str = join!(
    ":",
    "ecdsa_secp256r1_sha256",
    "rsa_pss_rsae_sha256",
    "rsa_pkcs1_sha256",
    "ecdsa_secp384r1_sha384",
    "rsa_pss_rsae_sha384",
    "rsa_pkcs1_sha384",
    "rsa_pss_rsae_sha512",
    "rsa_pkcs1_sha512"
);

/// Chromium supports both Brotli and Zstd for certificate compression.
pub const CERT_COMPRESSION_ALGORITHM: &[CertificateCompressionAlgorithm] = &[
    CertificateCompressionAlgorithm::BROTLI,
    CertificateCompressionAlgorithm::ZSTD,
];

#[derive(TypedBuilder)]
pub struct ChromiumTlsConfig {
    #[builder(default = CURVES)]
    curves: &'static str,

    #[builder(default = SIGALGS_LIST)]
    sigalgs_list: &'static str,

    #[builder(default = CIPHER_LIST)]
    cipher_list: &'static str,

    #[builder(default = AlpsProtocol::HTTP2, setter(into))]
    alps_protos: AlpsProtocol,

    /// Chromium 131+ uses new ALPS codepoint
    #[builder(default = true)]
    alps_use_new_codepoint: bool,

    #[builder(default = true, setter(into))]
    enable_ech_grease: bool,

    #[builder(default = true, setter(into))]
    permute_extensions: bool,

    #[builder(default = true, setter(into))]
    pre_shared_key: bool,

    /// Chromium only sends key shares for the first 2 named groups.
    #[builder(default = 2u8)]
    key_shares_limit: u8,
}

impl From<ChromiumTlsConfig> for TlsOptions {
    fn from(val: ChromiumTlsConfig) -> Self {
        TlsOptions::builder()
            // GREASE is always enabled in Chromium
            .grease_enabled(true)
            // Certificate transparency and OCSP
            .enable_ocsp_stapling(true)
            .enable_signed_cert_timestamps(true)
            // Cipher configuration
            .curves_list(val.curves)
            .sigalgs_list(val.sigalgs_list)
            .cipher_list(val.cipher_list)
            // TLS versions
            .min_tls_version(TlsVersion::TLS_1_2)
            .max_tls_version(TlsVersion::TLS_1_3)
            // Extension settings
            .permute_extensions(val.permute_extensions)
            .pre_shared_key(val.pre_shared_key)
            .enable_ech_grease(val.enable_ech_grease)
            // Key shares limit (Chromium only sends first 2)
            .key_shares_limit(val.key_shares_limit)
            // ALPS
            .alps_protocols([val.alps_protos])
            .alps_use_new_codepoint(val.alps_use_new_codepoint)
            // Hardware AES
            .aes_hw_override(true)
            // Certificate compression
            .certificate_compression_algorithms(CERT_COMPRESSION_ALGORITHM)
            .build()
    }
}

macro_rules! tls_options {
    () => {
        tls::ChromiumTlsConfig::builder().build().into()
    };
}
