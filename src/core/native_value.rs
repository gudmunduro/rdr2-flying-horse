use std::ffi::CStr;

use super::types::Vector3;

pub struct NativeValues<const N: usize>(pub [u64; N]);

impl<const N: usize> NativeValues<N> {
    pub fn values(&self) -> &[u64] {
        &self.0
    }

    fn single(v: u64) -> NativeValues<1> {
        NativeValues([v])
    }
}

impl From<i32> for NativeValues<1> {
    fn from(value: i32) -> Self {
        Self::single(value as u64)
    }
}
impl From<&i32> for NativeValues<1> {
    fn from(value: &i32) -> Self {
        Self::single(value as *const i32 as u64)
    }
}
impl From<&mut i32> for NativeValues<1> {
    fn from(value: &mut i32) -> Self {
        Self::single(value as *mut i32 as u64)
    }
}
impl From<i64> for NativeValues<1> {
    fn from(value: i64) -> Self {
        Self::single(value as u64)
    }
}
impl From<&u32> for NativeValues<1> {
    fn from(value: &u32) -> Self {
        Self::single(value as *const u32 as u64)
    }
}
impl From<&mut u32> for NativeValues<1> {
    fn from(value: &mut u32) -> Self {
        Self::single(value as *mut u32 as u64)
    }
}
impl From<u32> for NativeValues<1> {
    fn from(value: u32) -> Self {
        Self::single(value as u64)
    }
}
impl From<f32> for NativeValues<1> {
    fn from(value: f32) -> Self {
        Self::single(value.to_bits() as u64)
    }
}
impl From<&f32> for NativeValues<1> {
    fn from(value: &f32) -> Self {
        Self::single(value as *const f32 as u64)
    }
}
impl From<&mut f32> for NativeValues<1> {
    fn from(value: &mut f32) -> Self {
        Self::single(value as *mut f32 as u64)
    }
}
impl From<bool> for NativeValues<1> {
    fn from(value: bool) -> Self {
        Self::single(value as u64)
    }
}
impl From<&bool> for NativeValues<1> {
    fn from(value: &bool) -> Self {
        Self::single(value as *const bool as u64)
    }
}
impl From<&mut bool> for NativeValues<1> {
    fn from(value: &mut bool) -> Self {
        Self::single(value as *mut bool as u64)
    }
}
impl From<&Vector3> for NativeValues<1> {
    fn from(value: &Vector3) -> Self {
        Self::single(value as *const Vector3 as u64)
    }
}
impl From<&mut Vector3> for NativeValues<1> {
    fn from(value: &mut Vector3) -> Self {
        Self::single(value as *mut Vector3 as u64)
    }
}
impl From<&CStr> for NativeValues<1> {
    fn from(value: &CStr) -> Self {
        Self::single(value.as_ptr() as u64)
    }
}
impl From<&mut CStr> for NativeValues<1> {
    fn from(value: &mut CStr) -> Self {
        Self::single(value.as_ptr() as u64)
    }
}
impl From<Vector3> for NativeValues<3> {
    fn from(value: Vector3) -> Self {
        Self([value.x.to_bits() as u64, value.y.to_bits() as u64, value.z.to_bits() as u64])
    }
}
impl<T, const N: usize> From<Option<T>> for NativeValues<N> where NativeValues<N>: From<T>  {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => NativeValues::from(v),
            None => Self([0; N])
        }
    }
}