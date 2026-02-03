//! Chromium-accurate HTTP/2 configuration.
//!
//! These settings match the exact HTTP/2 SETTINGS frame values from Chromium source code.
//! Source: net/http/http_network_session.cc, net/spdy/spdy_session.h

/// Chromium's default stream dependency for HEADERS frames.
/// Weight 220 (219 + 1), exclusive dependency on stream 0.
macro_rules! headers_stream_dependency {
    () => {
        StreamDependency::new(StreamId::zero(), 219, true)
    };
}

/// Chromium's pseudo-header order in HEADERS frames.
/// Source: net/spdy/spdy_http_utils.cc
macro_rules! pseudo_order {
    () => {
        PseudoOrder::builder()
            .extend([
                PseudoId::Method,
                PseudoId::Authority,
                PseudoId::Scheme,
                PseudoId::Path,
            ])
            .build()
    };
}

/// Chromium's SETTINGS frame order.
/// This order is fingerprinted by anti-bot services like Akamai.
macro_rules! settings_order {
    () => {
        SettingsOrder::builder()
            .extend([
                SettingId::HeaderTableSize,
                SettingId::EnablePush,
                SettingId::MaxConcurrentStreams,
                SettingId::InitialWindowSize,
                SettingId::MaxFrameSize,
                SettingId::MaxHeaderListSize,
                SettingId::EnableConnectProtocol,
                SettingId::NoRfc7540Priorities,
            ])
            .build()
    };
}

/// Chromium production HTTP/2 options.
///
/// These values match production Chromium/Chrome:
/// - `initial_window_size`: 6,291,456 (6 MB) - kSpdyStreamMaxRecvWindowSize
/// - `initial_connection_window_size`: 15,728,640 (15 MB) - kSpdySessionMaxRecvWindowSize
/// - `header_table_size`: 65,536 (64 KB) - kSpdyMaxHeaderTableSize
/// - `max_header_list_size`: 262,144 (256 KB) - kSpdyMaxHeaderListSize
/// - `max_concurrent_streams`: 100 - kInitialMaxConcurrentStreams
macro_rules! http2_options {
    () => {
        Http2Options::builder()
            // Chromium production values (net/http/http_network_session.cc)
            .initial_window_size(6291456)          // 6 MB
            .initial_connection_window_size(15728640)  // 15 MB
            // Chromium's header table size (net/http/http_network_session.h)
            .header_table_size(65536)              // 64 KB
            // Chromium's max header list size
            .max_header_list_size(262144)          // 256 KB
            // Chromium's max concurrent streams (net/spdy/spdy_session.h)
            .max_concurrent_streams(100)
            // Push is disabled in Chromium
            .enable_push(false)
            // Stream dependency for HEADERS
            .headers_stream_dependency(headers_stream_dependency!())
            // Pseudo-header order
            .headers_pseudo_order(pseudo_order!())
            // Settings frame order
            .settings_order(settings_order!())
            .build()
    };
}
