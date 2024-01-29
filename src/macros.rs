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
