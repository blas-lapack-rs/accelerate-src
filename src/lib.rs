/*

This file creates an empty library for a normal build and has some tests for a
test build.

For a normal build, this file is required so that `extern crate
accelerate_provider` works in packages that depend on this one. And `extern
crate accelerate_provider` is necessary so that the linker flag specified in
build.rs is passed to those packages.

*/

#[cfg(test)]
extern crate libc;

#[allow(non_camel_case_types)]
#[cfg(test)]
mod tests {
    use libc::{c_float, size_t, ptrdiff_t};

    extern "C" {
        // Vector-scalar multiply.
        fn vDSP_vsmul(vDSP_input1: *const c_float, vDSP_stride1: ptrdiff_t,
                      vDSP_input2: *const c_float, vDSP_result: *mut c_float,
                      vDSP_strideResult: ptrdiff_t, vDSP_size: size_t);
    }

    fn approx_eq(x: c_float, y: c_float) -> bool {
        let eps = 1e-10;
        (x - y).abs() < (eps * y).abs()
    }

    #[test]
    fn check_linking() {
        let input = [1.0, 2.0, 3.0];
        let factor = 5.0;
        let mut output = [0.0, 0.0, 0.0];
        unsafe {
            vDSP_vsmul(input.as_ptr(), 1, &factor, output.as_mut_ptr(), 1, 3);
        }
        assert!(approx_eq(output[0], 5.0));
        assert!(approx_eq(output[1], 10.0));
        assert!(approx_eq(output[2], 15.0));
    }
}
