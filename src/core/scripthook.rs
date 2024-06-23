use crate::core::libscripthook::script_wait;
use crate::core::native_value::NativeValues;

macro_rules! invoke {
    ($hash:expr) => {
        unsafe {
            native_init($hash);
            native_call_2()
        }
    };
    ($hash:expr, $($arg:expr),*) => {
        unsafe {
            native_init($hash);
            $(
                native_push($arg);
            )*
            native_call_2()
        }
    };
}

macro_rules! invoke_ignore {
    ($hash:expr) => {
        unsafe {
            native_init($hash);
            native_call();
        }
    };
    ($hash:expr, $($arg:expr),*) => {
        unsafe {
            native_init($hash);
            $(
                native_push($arg);
            )*
            native_call();
        }
    };
}

pub(crate) use invoke_ignore;
pub(crate) use invoke;
pub(crate) use crate::core::libscripthook::{native_call, native_init, native_push_64};

pub unsafe fn native_push<'a, const N: usize>(value: impl Into<NativeValues<N>>) {
    let values: NativeValues<N> = value.into();
    for value in values.0 {
        native_push_64(value);
    }
}

pub unsafe fn native_call_2<R: Copy>() -> R {
    *(native_call() as *mut R)
}

pub fn wait(time: u32) {
    unsafe {
        script_wait(time);
    }
}