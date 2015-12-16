/// Get out & err.
#[macro_export] macro_rules! get_out {
    ( $out:ident <- $rexpr:expr, $err:ident, $r:expr ) => {
        unsafe {
            let mut $out = $rexpr;
            let mut $err = ::std::mem::uninitialized();
            if $r != 0 {
                Ok($out)
            } else {
                Err($err)
            }
        }
    };
    ( $out:ident, $err:ident, $r:expr ) => {
        get_out!(
            $out <- ::std::mem::uninitialized(),
            $err,
            $r
        )
    }
}


/// try err, from rstox.
#[macro_export] macro_rules! try_err {
    ( $err:ident, $r:expr ) => {
        unsafe {
            let mut $err = ::std::mem::uninitialized();
            let res = $r;
            match $err as ::libc::c_uint {
                0 => (),
                _ => return Err($err)
            };
            res
        }
    }
}
