/*

This file creates an empty library for a normal build and has some tests for a
test build.

For a normal build, this file is required so that `extern crate
accelerate_provider` works in packages that depend on this one. And `extern
crate accelerate_provider` is necessary so that the linker flag specified in
build.rs is passed to those packages.

*/

#![allow(non_camel_case_types)]

#[cfg(test)]
extern crate libc;

#[cfg(test)]
mod tests {
    use libc::{c_int, c_float};

    // FIXME: I'm not sure if these next two lines are correct.
    type vDSP_Stride = c_int;
    type vDSP_Length = c_int;

    extern "C" {
        // Multiply vector __vDSP_A by scalar __vDSP_B and place result in __vDSP_C.
        pub fn vDSP_vsmul(__vDSP_A: *const c_float, __vDSP_IA: vDSP_Stride, __vDSP_B: *const c_float, __vDSP_C: *mut c_float, __vDSP_IC: vDSP_Stride, __vDSP_N: vDSP_Length );
    }

    fn approx_eq(x: f32, y:f32) -> bool {
        let eps = 1e-10;
        (x - y).abs() < (eps * y).abs()
    }

    #[test]
    fn check_linking() {
        let input : [f32; 3] = [1.0,2.0,3.0];
        let mut output : [f32; 3] = [0.0, 0.0, 0.0];
        let factor: f32 = 5.0;
        // multiply each element in [1,2,3] by 5
        unsafe {
            vDSP_vsmul(input.as_ptr(), 1, &factor, output.as_mut_ptr(), 1, 3);
        }
        assert!(approx_eq(output[0], 5.0));
        assert!(approx_eq(output[1], 10.0));
        assert!(approx_eq(output[2], 15.0));
    }

}
