pub mod log {

macro_rules! trace {
    ($str:literal) => (println!(concat!("[TRACE] ", $str)));
    ($str:literal, $($arg:tt)*) => (println!(concat!("[TRACE] ", $str), $($arg)*));
}

macro_rules! info {
    ($str:literal) => (println!(concat!("[INFO] ", $str)));
    ($str:literal, $($arg:tt)*) => (println!(concat!("[INFO] ", $str), $($arg)*));
}

macro_rules! warning {
    ($str:literal) => (println!(concat!("[WARNING] ", $str)));
    ($str:literal, $($arg:tt)*) => (println!(concat!("[WARNING] ", $str), $($arg)*));
}

macro_rules! error {
    ($str:literal) => (println!(concat!("[ERROR] ", $str)));
    ($str:literal, $($arg:tt)*) => (println!(concat!("[ERROR] ", $str), $($arg)*));
}

macro_rules! fatal {
    ($str:literal) => (println!(concat!("[FATAL] ", $str)));
    ($str:literal, $($arg:tt)*) => (println!(concat!("[FATAL] ", $str), $($arg)*));
}

pub(crate) use trace;
pub(crate) use info;
pub(crate) use warning;
pub(crate) use error;
pub(crate) use fatal;

}
