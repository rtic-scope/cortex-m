/// Macro for sending a formatted string through an ITM channel
#[macro_export]
macro_rules! iprint {
    ($channel:expr, $s:expr) => {
        $crate::itm::write_str($channel, $s);
    };
    ($channel:expr, $($arg:tt)*) => {
        $crate::itm::write_fmt($channel, format_args!($($arg)*));
    };
}

/// Macro for sending a formatted string through an ITM channel, with a newline.
#[macro_export]
macro_rules! iprintln {
    ($channel:expr) => {
        $crate::itm::write_str($channel, "\n");
    };
    ($channel:expr, $fmt:expr) => {
        $crate::itm::write_str($channel, concat!($fmt, "\n"));
    };
    ($channel:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::itm::write_fmt($channel, format_args!(concat!($fmt, "\n"), $($arg)*));
    };
}

/// Macro to create a mutable reference to a statically allocated value
///
/// This macro returns a value with type `Option<&'static mut $ty>`. `Some($expr)` will be returned
/// the first time the macro is executed; further calls will return `None`. To avoid `unwrap`ping a
/// `None` variant the caller must ensure that the macro is called from a function that's executed
/// at most once in the whole lifetime of the program.
///
/// # Note
/// This macro is unsound on multi core systems.
///
/// # Example
///
/// ``` no_run
/// use cortex_m::singleton;
///
/// fn main() {
///     // OK if `main` is executed only once
///     let x: &'static mut bool = singleton!(: bool = false).unwrap();
///
///     let y = alias();
///     // BAD this second call to `alias` will definitively `panic!`
///     let y_alias = alias();
/// }
///
/// fn alias() -> &'static mut bool {
///     singleton!(: bool = false).unwrap()
/// }
/// ```
#[macro_export]
macro_rules! singleton {
    (: $ty:ty = $expr:expr) => {
        $crate::interrupt::free(|_| {
            static mut VAR: Option<$ty> = None;

            #[allow(unsafe_code)]
            let used = unsafe { VAR.is_some() };
            if used {
                None
            } else {
                let expr = $expr;

                #[allow(unsafe_code)]
                unsafe {
                    VAR = Some(expr)
                }

                #[allow(unsafe_code)]
                unsafe {
                    VAR.as_mut()
                }
            }
        })
    };
}

/// ``` compile_fail
/// use cortex_m::singleton;
///
/// fn foo() {
///     // check that the call to `uninitialized` requires unsafe
///     singleton!(: u8 = std::mem::uninitialized());
/// }
/// ```
#[allow(dead_code)]
const CFAIL: () = ();

/// ```
/// #![deny(unsafe_code)]
/// use cortex_m::singleton;
///
/// fn foo() {
///     // check that calls to `singleton!` don't trip the `unsafe_code` lint
///     singleton!(: u8 = 0);
/// }
/// ```
#[allow(dead_code)]
const CPASS: () = ();
