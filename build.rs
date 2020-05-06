use std::env;
use version_check as rustc;

const MSRV: &str = "1.36.0";

macro_rules! cfg_emit {
    ($s:ident) => {
        println!(concat!("cargo:rustc-cfg=", stringify!($s)));
    };
}

macro_rules! cfg_aliases {
    ($($alias:ident = { $($tokens:tt)* })*) => {$(
        #[cfg($($tokens)*)]
        cfg_emit!($alias);
    )*};
}

macro_rules! warning {
    ($($s:tt)*) => {
        println!("cargo:warning={}", format_args!($($s)*));
    };
}

fn main() {
    cfg_aliases! {
        // Simple aliases for feature gates
        std = { feature = "std" }
        rand = { feature = "rand" }
        serde = { feature = "serde" }
        macros = { feature = "macros" }
        local_offset = { feature = "local-offset" }
        docs = { feature = "__doc" }

        // OS aliases
        unix = { target_family = "unix" }
        windows = { target_family = "windows" }

        // Unix-specific API extensions
        // Using this outside of a `#[cfg(unix)]` block is a logical error.
        gmtoff_ext = { not(any(target_os = "solaris", target_os = "illumos")) }
    };

    // Are we compiling with `cargo web`?
    if env::var("COMPILING_UNDER_CARGO_WEB") == Ok("1".into()) {
        cfg_emit!(cargo_web);
    }

    // Warn if the version is below MSRV.
    if !rustc::is_min_version(MSRV).unwrap_or(false) {
        warning!(
            "The time crate has a minimum supported rust version of {}.",
            MSRV
        );
    }

    // Warn if the `__doc` feature is used on stable or beta.
    if !rustc::Channel::read().map_or(false, |channel| channel.supports_features()) {
        #[cfg(feature = "__doc")]
        warning!(
            "The `__doc` feature requires a nightly compiler, and is intended for internal usage \
             only."
        );
    }

    // ==== features that affect runtime directly ====

    // `#[non_exhaustive]` was stabilized in 1.40.0.
    if rustc::is_min_version("1.40.0").unwrap_or(false) {
        cfg_emit!(supports_non_exhaustive);
    }

    // `(-5).abs()` is `const`-capable beginning in 1.39.0.
    if rustc::is_min_version("1.39.0").unwrap_or(false) {
        cfg_emit!(const_num_abs);
    }
}
