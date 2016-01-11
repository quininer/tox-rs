macro_rules! out {
    ( out $out:ident <- $rexpr:expr, $err:ident, $exp:expr ) => {
        unsafe {
            let mut $out = $rexpr;
            let mut $err = ::std::mem::uninitialized();
            if $exp {
                Ok($out)
            } else {
                Err($err)
            }
        }
    };
    ( out $out:ident, $err:ident, $exp:expr ) => {
        out!( out
            $out <- ::std::mem::uninitialized(),
            $err,
            $exp
        )
    };
    ( bool $err:ident, $exp:expr ) => {
        unsafe {
            let mut $err = ::std::mem::uninitialized();
            match $exp {
                true => Ok(()),
                false => Err($err)
            }
        }
    };
    ( ( num <- $num:ty ) $err:ident, $exp:expr ) => {
        unsafe {
            const MAX: $num = !0;
            let mut $err = ::std::mem::uninitialized();
            match $exp {
                MAX => Err($err),
                out @ _ => Ok(out)
            }
        }
    };
    ( num $err:ident, $exp:expr ) => {
        out!( ( num <- ::libc::uint32_t ) $err, $exp )
    };
    ( err $err:ident, $exp:expr ) => {
        unsafe {
            let mut $err = ::std::mem::uninitialized();
            let out = $exp;
            match $err as ::libc::c_int {
                0 => Ok(out),
                _ => Err($err)
            }
        }
    };
    ( get $out:ident <- $rexpr:expr, $exp:expr ) => {
        unsafe {
            let mut $out = $rexpr;
            $exp;
            $out
        }
    }
}

macro_rules! vec_with {
    ( $len:expr ) => {{
        let len = $len;
        let mut out = Vec::with_capacity(len);
        out.set_len(len);
        out
    }}
}
