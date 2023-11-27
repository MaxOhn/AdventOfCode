#[macro_export]
macro_rules! get {
    ($slice:expr, $idx:expr) => {
        unsafe { *$slice.get_unchecked($idx) }
    };
}
#[macro_export]
macro_rules! get_ref {
    ($slice:expr, $idx:expr) => {
        unsafe { $slice.get_unchecked($idx) }
    };
}

#[macro_export]
macro_rules! get_mut {
    ($slice:expr, $idx:expr) => {
        unsafe { $slice.get_unchecked_mut($idx) }
    };
}

#[macro_export]
macro_rules! set {
    ($slice:expr, $idx:expr, $val:expr) => {
        unsafe { *get_mut!($slice, $idx) = $val }
    };
}

#[macro_export]
macro_rules! swap {
    ($a:expr, $b:expr) => {
        std::mem::swap($a, $b)
    };
}
