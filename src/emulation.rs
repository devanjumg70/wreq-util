mod device;
#[cfg(feature = "emulation-rand")]
mod rand;

use device::chrome::*;
#[cfg(feature = "emulation-serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "emulation-rand")]
use strum_macros::VariantArray;
use typed_builder::TypedBuilder;

macro_rules! define_enum {
    (
        $(#[$meta:meta])*
        with_dispatch,
        $name:ident, $default_variant:ident,
        $(
            $variant:ident => ($rename:expr, $emulation_fn:path)
        ),* $(,)?
    ) => {
        $(#[$meta])*
        #[non_exhaustive]
        #[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "emulation-rand", derive(VariantArray))]
        #[cfg_attr(feature = "emulation-serde", derive(Deserialize, Serialize))]
        pub enum $name {
            $(
                #[cfg_attr(feature = "emulation-serde", serde(rename = $rename))]
                $variant,
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                $name::$default_variant
            }
        }

        impl $name {
            pub fn into_emulation(self, opt: EmulationOption) -> wreq::Emulation {
                match self {
                    $(
                        $name::$variant => $emulation_fn(opt),
                    )*
                }
            }
        }
    };

    (
        $(#[$meta:meta])*
        plain,
        $name:ident, $default_variant:ident,
        $(
            $variant:ident => $rename:expr
        ),* $(,)?
    ) => {
        $(#[$meta])*
        #[non_exhaustive]
        #[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "emulation-rand", derive(VariantArray))]
        #[cfg_attr(feature = "emulation-serde", derive(Deserialize, Serialize))]
        pub enum $name {
            $(
                #[cfg_attr(feature = "emulation-serde", serde(rename = $rename))]
                $variant,
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                $name::$default_variant
            }
        }
    };
}

define_enum!(
    /// Represents different browser versions for emulation.
    ///
    /// The `Emulation` enum provides variants for different browser versions that can be used
    /// to emulation HTTP requests. Each variant corresponds to a specific browser version.
    ///
    /// # Naming Convention
    ///
    /// The naming convention for the variants follows the pattern `browser_version`, where
    /// `browser` is the name of the browser (e.g., `chrome`, `firefox`, `safari`) and `version`
    /// is the version number. For example, `Chrome100` represents Chrome version 100.
    ///
    /// The serialized names of the variants use underscores to separate the browser name and
    /// version number, following the pattern `browser_version`. For example, `Chrome100` is
    /// serialized as `"chrome_100"`.
    with_dispatch,
    Emulation, Chrome100,

    // Chrome versions
    Chrome100 => ("chrome_100", v100::emulation),
    Chrome101 => ("chrome_101", v101::emulation),
    Chrome104 => ("chrome_104", v104::emulation),
    Chrome105 => ("chrome_105", v105::emulation),
    Chrome106 => ("chrome_106", v106::emulation),
    Chrome107 => ("chrome_107", v107::emulation),
    Chrome108 => ("chrome_108", v108::emulation),
    Chrome109 => ("chrome_109", v109::emulation),
    Chrome110 => ("chrome_110", v110::emulation),
    Chrome114 => ("chrome_114", v114::emulation),
    Chrome116 => ("chrome_116", v116::emulation),
    Chrome117 => ("chrome_117", v117::emulation),
    Chrome118 => ("chrome_118", v118::emulation),
    Chrome119 => ("chrome_119", v119::emulation),
    Chrome120 => ("chrome_120", v120::emulation),
    Chrome123 => ("chrome_123", v123::emulation),
    Chrome124 => ("chrome_124", v124::emulation),
    Chrome126 => ("chrome_126", v126::emulation),
    Chrome127 => ("chrome_127", v127::emulation),
    Chrome128 => ("chrome_128", v128::emulation),
    Chrome129 => ("chrome_129", v129::emulation),
    Chrome130 => ("chrome_130", v130::emulation),
    Chrome131 => ("chrome_131", v131::emulation),
    Chrome132 => ("chrome_132", v132::emulation),
    Chrome133 => ("chrome_133", v133::emulation),
    Chrome134 => ("chrome_134", v134::emulation),
    Chrome135 => ("chrome_135", v135::emulation),
    Chrome136 => ("chrome_136", v136::emulation),
    Chrome137 => ("chrome_137", v137::emulation),
    Chrome138 => ("chrome_138", v138::emulation),
    Chrome139 => ("chrome_139", v139::emulation),
    Chrome140 => ("chrome_140", v140::emulation),
    Chrome141 => ("chrome_141", v141::emulation),
    Chrome142 => ("chrome_142", v142::emulation),
    Chrome143 => ("chrome_143", v143::emulation),
    Chrome144 => ("chrome_144", v144::emulation),
    Chrome145 => ("chrome_145", v145::emulation),







);

/// ======== Emulation impls ========
impl wreq::EmulationFactory for Emulation {
    #[inline]
    fn emulation(self) -> wreq::Emulation {
        EmulationOption::builder()
            .emulation(self)
            .build()
            .emulation()
    }
}

define_enum!(
    /// Represents different operating systems for emulation.
    ///
    /// The `EmulationOS` enum provides variants for different operating systems that can be used
    /// to emulation HTTP requests. Each variant corresponds to a specific operating system.
    ///
    /// # Naming Convention
    ///
    /// The naming convention for the variants follows the pattern `os_name`, where
    /// `os_name` is the name of the operating system (e.g., `windows`, `macos`, `linux`, `android`, `ios`).
    ///
    /// The serialized names of the variants use lowercase letters to represent the operating system names.
    /// For example, `Windows` is serialized as `"windows"`.
    plain,
    EmulationOS, MacOS,
    Windows => "windows",
    MacOS => "macos",
    Linux => "linux",
    Android => "android",
    IOS => "ios"
);

/// ======== EmulationOS impls ========
impl EmulationOS {
    #[inline]
    const fn platform(&self) -> &'static str {
        match self {
            EmulationOS::MacOS => "\"macOS\"",
            EmulationOS::Linux => "\"Linux\"",
            EmulationOS::Windows => "\"Windows\"",
            EmulationOS::Android => "\"Android\"",
            EmulationOS::IOS => "\"iOS\"",
        }
    }

    #[inline]
    const fn is_mobile(&self) -> bool {
        matches!(self, EmulationOS::Android | EmulationOS::IOS)
    }
}

/// Represents the configuration options for emulating a browser and operating system.
///
/// The `EmulationOption` struct allows you to configure various aspects of browser and OS
/// emulation, including the browser version, operating system, and whether to skip certain features
/// like HTTP/2 or headers.
///
/// This struct is typically used to build an `EmulationProvider` that can be applied to HTTP
/// clients for making requests that mimic specific browser and OS configurations.
///
/// # Fields
///
/// - `emulation`: The browser version to emulate. Defaults to `Emulation::default()`.
/// - `emulation_os`: The operating system to emulate. Defaults to `EmulationOS::default()`.
/// - `skip_http2`: Whether to skip HTTP/2 support. Defaults to `false`.
/// - `skip_headers`: Whether to skip adding default headers. Defaults to `false`.
#[derive(Default, Clone, TypedBuilder)]
pub struct EmulationOption {
    /// The browser version to emulation.
    #[builder(default)]
    emulation: Emulation,

    /// The operating system.
    #[builder(default)]
    emulation_os: EmulationOS,

    /// Whether to skip HTTP/2.
    #[builder(default = false)]
    skip_http2: bool,

    /// Whether to skip headers.
    #[builder(default = false)]
    skip_headers: bool,
}

/// ======== EmulationOption impls ========
impl wreq::EmulationFactory for EmulationOption {
    #[inline]
    fn emulation(self) -> wreq::Emulation {
        self.emulation.into_emulation(self)
    }
}
