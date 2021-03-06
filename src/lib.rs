use {
    ndarray::{Array2, Array3, ArrayView2, ArrayView3},
    simdeez::{avx2::*, scalar::*, sse2::*, sse41::*, *},
};

// If you want your SIMD function to use use runtime feature detection to call
// the fastest available version, use the simd_runtime_generate macro:
simd_runtime_generate!(
    pub fn distance(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        let mut result: Vec<f32> = Vec::with_capacity(x1.len());
        result.set_len(x1.len()); // for efficiency

        // Operations have to be done in terms of the vector width
        // so that it will work with any size vector.
        // the width of a vector type is provided as a constant
        // so the compiler is free to optimize it more.
        // S::VF32_WIDTH is a constant, 4 when using SSE, 8 when using AVX2, etc
        for i in (0..x1.len()).step_by(S::VF32_WIDTH) {
            //load data from your vec into a SIMD value
            let xv1 = S::loadu_ps(&x1[i]);
            let yv1 = S::loadu_ps(&y1[i]);
            let xv2 = S::loadu_ps(&x2[i]);
            let yv2 = S::loadu_ps(&y2[i]);

            // Use the usual intrinsic syntax if you prefer
            let mut xdiff = S::sub_ps(xv1, xv2);
            // Or use operater overloading if you like
            let mut ydiff = yv1 - yv2;
            xdiff *= xdiff;
            ydiff *= ydiff;
            let distance = S::sqrt_ps(xdiff + ydiff);
            // Store the SIMD value into the result vec
            S::storeu_ps(&mut result[i], distance);
        }
        result
    }
);

simd_runtime_generate!(
    pub fn muladd_arr3(a: &mut Array3<f64>, b: ArrayView3<f64>, c: ArrayView3<f64>) {
        let a = a.as_slice_memory_order_mut().unwrap();
        let b = b.as_slice_memory_order().unwrap();

        for i in (0..a.len()).step_by(S::VF32_WIDTH) {
            let a0 = S::loadu_pd(&a[i]);
            let b0 = S::loadu_pd(&b[i]);
            S::storeu_pd(&mut a[i], a0 + b0);
        }
    }
);

simd_runtime_generate!(
    pub fn muladd_arr2(a: &mut Array2<f64>, b: ArrayView2<f64>, c: ArrayView2<f64>) {
        let a = a.as_slice_memory_order_mut().unwrap();
        let b = b.as_slice_memory_order().unwrap();

        for i in (0..a.len()).step_by(S::VF32_WIDTH) {
            let a0 = S::loadu_pd(&a[i]);
            let b0 = S::loadu_pd(&b[i]);
            S::storeu_pd(&mut a[i], a0 + b0);
        }
    }
);
