/// C function out wrapper
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
            if $exp {
                Ok(())
            } else {
                Err($err)
            }
        }
    };
    ( ( num <- $num:ty ) $err:ident, $exp:expr ) => {
        unsafe {
            const MAX: $num = !0;
            let mut $err = ::std::mem::uninitialized();
            match $exp {
                MAX => Err($err),
                out => Ok(out)
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
    };
    ( get $out:ident, $exp:expr ) => {
        out!( get $out <- ::std::mem::uninitialized(), $exp);
    }
}

/// Fixed size Vec
macro_rules! vec_with {
    ( $len:expr ) => {{
        let len = $len;
        let mut out = Vec::with_capacity(len);
        out.set_len(len);
        out
    }}
}

/// Event Callback
macro_rules! callback {
    ( ( $prefix:ident, $core:expr, $tx:expr ), $( $event:ident ),* ) => {{
        use super::ffi::*;
        $(
            concat_idents!($prefix, _callback_, $event)($core, concat_idents!(on_, $event), $tx);
        )*
    }}
}

/// FFI Struct rename
macro_rules! use_as {
    ( $( $old:ident as $new:ident ),* ) => {
        $(
            pub use super::ffi::$old as $new;
        )*
    }
}
