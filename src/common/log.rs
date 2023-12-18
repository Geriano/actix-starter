#[macro_export]
macro_rules! log_info {
    ($source:ty, $format:tt, $($arg:tt)*) => {
        println!(
            "[\x1b[036;1mINFO\x1b[0m][\x1b[030;1m{}\x1b[0m][\x1b[037;1m{}::{}({})\x1b[0m] {}",
            crate::common::time::now().format("%Y-%m-%d %H:%M:%S"),
            std::module_path!(),
            stringify!($source),
            std::line!(),
            format!($format, $($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_err {
    ($source:ty, $format:tt, $($arg:tt)*) => {
        eprintln!(
            "[\x1b[031;1mERROR\x1b[0m][\x1b[030;1m{}\x1b[0m][\x1b[037;1m{}::{}({})\x1b[0m] {}",
            crate::common::time::now().format("%Y-%m-%d %H:%M:%S"),
            std::module_path!(),
            stringify!($source),
            std::line!(),
            format!($format, $($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_dbg {
    ($source:ty, $format:tt, $($arg:tt)*) => {
        #[cfg(not(release))]
        println!(
            "[\x1b[034;1mDEBUG\x1b[0m][\x1b[030;1m{}\x1b[0m][\x1b[037;1m{}::{}({})\x1b[0m] {}",
            crate::common::time::now().format("%Y-%m-%d %H:%M:%S"),
            std::module_path!(),
            stringify!($source),
            std::line!(),
            format!($format, $($arg)*)
        )
    };
}

pub use log_dbg as debug;
pub use log_err as error;
pub use log_info as info;
