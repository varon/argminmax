#[cfg(feature = "half")]
use super::scalar_f16::scalar_argminmax_f16;
#[cfg(feature = "half")]
use half::f16;

pub trait ScalarArgMinMax<ScalarDType: Copy + PartialOrd> {
    fn argminmax(data: &[ScalarDType]) -> (usize, usize);
}

pub struct SCALAR;

// #[inline(always)] leads to poor performance on aarch64

// #[inline(never)]
pub fn scalar_argminmax<T: Copy + PartialOrd>(arr: &[T]) -> (usize, usize) {
    assert!(!arr.is_empty());
    let mut low_index: usize = 0;
    let mut high_index: usize = 0;
    // It is remarkably faster to iterate over the index and use get_unchecked
    // than using .iter().enumerate() (with a fold).
    let mut low: T = unsafe { *arr.get_unchecked(low_index) };
    let mut high: T = unsafe { *arr.get_unchecked(high_index) };
    for i in 0..arr.len() {
        let v: T = unsafe { *arr.get_unchecked(i) };
        if v < low {
            low = v;
            low_index = i;
        } else if v > high {
            high = v;
            high_index = i;
        }
    }
    (low_index, high_index)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
// #[inline(never)]
pub fn scalar_argminmax_fold<T: Copy + PartialOrd>(arr: &[T]) -> (usize, usize) {
    let minmax_tuple: (usize, T, usize, T) = arr.iter().enumerate().fold(
        (0usize, arr[0], 0usize, arr[0]),
        |(min_idx, min, max_idx, max), (idx, &item)| {
            if item < min {
                (idx, item, max_idx, max)
            } else if item > max {
                (min_idx, min, idx, item)
            } else {
                (min_idx, min, max_idx, max)
            }
        },
    );
    (minmax_tuple.0, minmax_tuple.2)
}

macro_rules! impl_scalar {
    ($func:ident, $($t:ty),*) =>
    {
        $(
            impl ScalarArgMinMax<$t> for SCALAR {
                fn argminmax(data: &[$t]) -> (usize, usize) {
                    $func(data)
                }
            }
        )*
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod scalar_x86 {
    use super::*;
    impl_scalar!(
        scalar_argminmax,
        i8,
        i16,
        i32,
        i64,
        u8,
        u16,
        u32,
        u64,
        f32,
        f64
    );
}
#[cfg(target_arch = "aarch64")]
mod scalar_aarch64 {
    use super::*;
    impl_scalar!(scalar_argminmax_fold, i8, i16, i32, i64, u8, u16, u32, u64);
    impl_scalar!(scalar_argminmax, f32, f64);
}
#[cfg(target_arch = "arm")]
mod scalar_arm {
    use super::*;
    impl_scalar!(scalar_argminmax_fold, i16, i64, u8, u16, u64, f64);
    impl_scalar!(scalar_argminmax, i8, u32, f32, i32);
}
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm"
)))]
mod scalar_generic {
    use super::*;
    impl_scalar!(
        scalar_argminmax,
        i8,
        i16,
        i32,
        i64,
        u8,
        u16,
        u32,
        u64,
        f32,
        f64
    );
}

#[cfg(feature = "half")]
impl_scalar!(scalar_argminmax_f16, f16);
