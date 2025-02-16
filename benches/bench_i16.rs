#![feature(stdsimd)]

extern crate dev_utils;

#[cfg(feature = "half")]
use argminmax::ArgMinMax;
use codspeed_criterion_compat::*;
use dev_utils::{config, utils};

use argminmax::{ScalarArgMinMax, SCALAR};
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use argminmax::{AVX2, AVX512, SIMD, SSE};
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use argminmax::{NEON, SIMD};

fn minmax_i16_random_array_long(c: &mut Criterion) {
    let n = config::ARRAY_LENGTH_LONG;
    let data: &[i16] = &utils::get_random_array::<i16>(n, i16::MIN, i16::MAX);
    c.bench_function("scalar_random_long_i16", |b| {
        b.iter(|| SCALAR::argminmax(black_box(data)))
    });
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("sse4.1") {
        c.bench_function("sse_random_long_i16", |b| {
            b.iter(|| unsafe { SSE::argminmax(black_box(data)) })
        });
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx2") {
        c.bench_function("avx2_random_long_i16", |b| {
            b.iter(|| unsafe { AVX2::argminmax(black_box(data)) })
        });
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx512bw") {
        c.bench_function("avx512_random_long_i16", |b| {
            b.iter(|| unsafe { AVX512::argminmax(black_box(data)) })
        });
    }
    #[cfg(target_arch = "arm")]
    if std::arch::is_arm_feature_detected!("neon") {
        c.bench_function("neon_random_long_i16", |b| {
            b.iter(|| unsafe { NEON::argminmax(black_box(data)) })
        });
    }
    #[cfg(target_arch = "aarch64")]
    if std::arch::is_aarch64_feature_detected!("neon") {
        c.bench_function("neon_random_long_i16", |b| {
            b.iter(|| unsafe { NEON::argminmax(black_box(data)) })
        });
    }
    c.bench_function("impl_random_long_i16", |b| {
        b.iter(|| black_box(data.argminmax()))
    });
}

fn minmax_i16_random_array_short(c: &mut Criterion) {
    let n = config::ARRAY_LENGTH_SHORT;
    let data: &[i16] = &utils::get_random_array::<i16>(n, i16::MIN, i16::MAX);
    c.bench_function("scalar_random_short_i16", |b| {
        b.iter(|| SCALAR::argminmax(black_box(data)))
    });
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("sse4.1") {
        c.bench_function("sse_random_short_i16", |b| {
            b.iter(|| unsafe { SSE::argminmax(black_box(data)) })
        });
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx2") {
        c.bench_function("avx2_random_short_i16", |b| {
            b.iter(|| unsafe { AVX2::argminmax(black_box(data)) })
        });
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx512bw") {
        c.bench_function("avx512_random_short_i16", |b| {
            b.iter(|| unsafe { AVX512::argminmax(black_box(data)) })
        });
    }
    #[cfg(target_arch = "arm")]
    if std::arch::is_arm_feature_detected!("neon") {
        c.bench_function("neon_random_short_i16", |b| {
            b.iter(|| unsafe { NEON::argminmax(black_box(data)) })
        });
    }
    #[cfg(target_arch = "aarch64")]
    if std::arch::is_aarch64_feature_detected!("neon") {
        c.bench_function("neon_random_short_i16", |b| {
            b.iter(|| unsafe { NEON::argminmax(black_box(data)) })
        });
    }
    c.bench_function("impl_random_short_i16", |b| {
        b.iter(|| black_box(data.argminmax()))
    });
}

fn minmax_i16_worst_case_array_long(c: &mut Criterion) {
    let n = config::ARRAY_LENGTH_LONG;
    let data: &[i16] = &utils::get_worst_case_array::<i16>(n, 1);
    c.bench_function("scalar_worst_long_i16", |b| {
        b.iter(|| SCALAR::argminmax(black_box(data)))
    });
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("sse4.1") {
        c.bench_function("sse_worst_long_i16", |b| {
            b.iter(|| unsafe { SSE::argminmax(black_box(data)) })
        });
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx2") {
        c.bench_function("avx2_worst_long_i16", |b| {
            b.iter(|| unsafe { AVX2::argminmax(black_box(data)) })
        });
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx512bw") {
        c.bench_function("avx512_worst_long_i16", |b| {
            b.iter(|| unsafe { AVX512::argminmax(black_box(data)) })
        });
    }
    #[cfg(target_arch = "arm")]
    if std::arch::is_arm_feature_detected!("neon") {
        c.bench_function("neon_worst_long_i16", |b| {
            b.iter(|| unsafe { NEON::argminmax(black_box(data)) })
        });
    }
    #[cfg(target_arch = "aarch64")]
    if std::arch::is_aarch64_feature_detected!("neon") {
        c.bench_function("neon_worst_long_i16", |b| {
            b.iter(|| unsafe { NEON::argminmax(black_box(data)) })
        });
    }
    c.bench_function("impl_worst_long_i16", |b| {
        b.iter(|| black_box(data.argminmax()))
    });
}

fn minmax_i16_worst_case_array_short(c: &mut Criterion) {
    let n = config::ARRAY_LENGTH_SHORT;
    let data: &[i16] = &utils::get_worst_case_array::<i16>(n, 1);
    c.bench_function("scalar_worst_short_i16", |b| {
        b.iter(|| SCALAR::argminmax(black_box(data)))
    });
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("sse4.1") {
        c.bench_function("sse_worst_short_i16", |b| {
            b.iter(|| unsafe { SSE::argminmax(black_box(data)) })
        });
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx2") {
        c.bench_function("avx2_worst_short_i16", |b| {
            b.iter(|| unsafe { AVX2::argminmax(black_box(data)) })
        });
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx512bw") {
        c.bench_function("avx512_worst_short_i16", |b| {
            b.iter(|| unsafe { AVX512::argminmax(black_box(data)) })
        });
    }
    #[cfg(target_arch = "arm")]
    if std::arch::is_arm_feature_detected!("neon") {
        c.bench_function("neon_worst_short_i16", |b| {
            b.iter(|| unsafe { NEON::argminmax(black_box(data)) })
        });
    }
    #[cfg(target_arch = "aarch64")]
    if std::arch::is_aarch64_feature_detected!("neon") {
        c.bench_function("neon_worst_short_i16", |b| {
            b.iter(|| unsafe { NEON::argminmax(black_box(data)) })
        });
    }
    c.bench_function("impl_worst_short_i16", |b| {
        b.iter(|| black_box(data.argminmax()))
    });
}

criterion_group!(
    benches,
    minmax_i16_random_array_long,
    // minmax_i16_random_array_short,
    // minmax_i16_worst_case_array_long,
    // minmax_i16_worst_case_array_short
);
criterion_main!(benches);
