use std::{fmt::Arguments, fs::OpenOptions, io::Write};
pub fn to_file(data: Arguments) {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./log")
        .expect("open log failed");
    write!(f, "{}", data.to_string()).expect("write log failed");
}
// creates debugging macro
#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => (if false { ::std::eprintln!($($arg)*); })
}

#[macro_export]
macro_rules! dserver {
    ($($arg:tt)*) => (if false { ::std::eprintln!($($arg)*); })
}

#[macro_export]
macro_rules! dclient {
    ($($arg:tt)*) => (if false { ::std::eprintln!($($arg)*); })
}

#[macro_export]
macro_rules! dinput{
    ($($arg:tt)*) => (if false { ::std::eprintln!($($arg)*); })
}

#[macro_export]
macro_rules! dfile{
    ($($arg:tt)*) => (if false {
        use crate::macros::to_file;
        to_file(format_args!($($arg)*));
    })
}
