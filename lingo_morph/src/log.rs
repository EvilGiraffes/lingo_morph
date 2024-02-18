// TODO remove
#![allow(unused)]

macro_rules! trace {
    ($($args:tt)+) => {{
        if cfg!(feature = "logging") {
            _log::trace!($($args)+);
        }
    }};
}

macro_rules! debug {
    ($($args:tt)+) => {{
        if cfg!(feature = "logging") {
            _log::debug!($($args)+);
        }
    }};
}

macro_rules! info {
    ($($args:tt)+) => {{
        if cfg!(feature = "logging") {
            _log::info!($($args)+);
        }
    }};
}

macro_rules! warn {
    ($($args:tt)+) => {{
        if cfg!(feature = "logging") {
            _log::warn!($($args)+);
        }
    }};
}

macro_rules! error {
    ($($args:tt)+) => {{
        if cfg!(feature = "logging") {
            _log::error!($($args)+);
        }
    }};
}

macro_rules! log_enabled {
    ($($args:tt)+) => {{
        if cfg!(feature = "logging") {
            _log::log_enabled!($($args)+);
        }
    }};
}
