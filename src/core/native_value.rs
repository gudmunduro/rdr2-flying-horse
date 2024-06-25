use std::ffi::CStr;

use super::types::Vector3;

macro_rules! add_native_type {
    ($from:ty) => {
        impl From<$from> for NativeValues<1> {
            fn from(v: $from) -> Self {
                Self::single(v as u64)
            }
        }
    };
    (ptr $from:ty) => {
        impl From<&$from> for NativeValues<1> {
            fn from(v: &$from) -> Self {
                Self::single(v as *const $from as u64)
            }
        }
    };
    (mut_ptr $from:ty) => {
        impl From<&mut $from> for NativeValues<1> {
            fn from(v: &mut $from) -> Self {
                Self::single(v as *mut $from as u64)
            }
        }
    };
    (all $from:ty) => {
        add_native_type!($from);
        add_native_type!(ptr $from);
        add_native_type!(mut_ptr $from);
    };
    ($from:ty, $convert:expr) => {
        impl From<$from> for NativeValues<1> {
            fn from(v: $from) -> Self {
                Self::single(($convert)(v))
            }
        }
    };
    (mul $from:ty, $count:expr, $convert:expr) => {
        impl From<$from> for NativeValues<$count> {
            fn from(v: $from) -> Self {
                Self(($convert)(v))
            }
        }
    };
}

pub struct NativeValues<const N: usize>(pub [u64; N]);

impl<const N: usize> NativeValues<N> {
    pub fn values(&self) -> &[u64] {
        &self.0
    }

    fn single(v: u64) -> NativeValues<1> {
        NativeValues([v])
    }
}

add_native_type!(all i32);
add_native_type!(i64);
add_native_type!(all u32);
add_native_type!(f32, |v: f32| v.to_bits() as u64);
add_native_type!(ptr f32);
add_native_type!(mut_ptr f32);
add_native_type!(all bool);
add_native_type!(&CStr, |v: &CStr| v.as_ptr() as u64);
add_native_type!(&mut CStr, |v: &mut CStr| v.as_ptr() as u64);
add_native_type!(mul Vector3, 3, |v: Vector3| [v.x.to_bits() as u64, v.y.to_bits() as u64, v.z.to_bits() as u64]);
add_native_type!(ptr Vector3);
add_native_type!(mut_ptr Vector3);

impl<T, const N: usize> From<Option<T>> for NativeValues<N> where NativeValues<N>: From<T>  {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => NativeValues::from(v),
            None => Self([0; N])
        }
    }
}
