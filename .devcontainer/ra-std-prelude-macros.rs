
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(no_inline)]
#[allow(deprecated)]
pub use core::{
    assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne, matches, todo, r#try,
    unimplemented, unreachable, write, writeln,
};

#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(no_inline)]
pub use crate::{dbg, eprint, eprintln, format, print, println, thread_local};

mod ambiguous_macros_only {
    mod vec {}
    mod panic {}
    #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
    pub use crate::*;
}
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(no_inline)]
pub use self::ambiguous_macros_only::{vec, panic};
