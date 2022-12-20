#[macro_export]
macro_rules! get {
    ( $( $slice:ident ).+ [$idx:expr] ) => {
        unsafe { * $( $slice. )* get_unchecked($idx) }
    };
}

#[macro_export]
macro_rules! get_mut {
    ( $( $slice:ident ).+ [$idx:expr] ) => {
        unsafe { $( $slice. )* get_unchecked_mut($idx) }
    };
}

#[macro_export]
macro_rules! set {
    ( $slice:ident[$idx:expr] $op:tt $val:expr ) => {
        unsafe { *$slice.get_unchecked_mut($idx) $op $val }
    };
}
