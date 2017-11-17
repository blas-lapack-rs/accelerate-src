#![allow(non_camel_case_types)]

extern crate accelerate_src;
extern crate libc;

use libc::{c_float, ptrdiff_t, size_t};

extern "C" {
    fn vDSP_vsmul(
        vDSP_input1: *const c_float,
        vDSP_stride1: ptrdiff_t,
        vDSP_input2: *const c_float,
        vDSP_result: *mut c_float,
        vDSP_strideResult: ptrdiff_t,
        vDSP_size: size_t,
    );
}

#[test]
fn link() {
    let input = [1.0, 2.0, 3.0];
    let factor = 5.0;
    let mut output = [0.0, 0.0, 0.0];
    unsafe {
        vDSP_vsmul(input.as_ptr(), 1, &factor, output.as_mut_ptr(), 1, 3);
    }
    assert!(close(output[0], 5.0));
    assert!(close(output[1], 10.0));
    assert!(close(output[2], 15.0));
}

fn close(x: c_float, y: c_float) -> bool {
    const EPS: c_float = 1e-10;
    (x - y).abs() < (EPS * y).abs()
}
