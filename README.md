# Accelerate Provider

[![Build Status](https://travis-ci.org/strawlab/accelerate-provider.svg)](https://travis-ci.org/strawlab/accelerate-provider)

This package links the Accelerate framework that Apple ships with Mac OS X.
Therefore, it provides functions from [BLAS][1] and [LAPACK][2], amongst others,
to [Rust][3]. MIT licensed.

## Where are all the FFI definitions?

This package only links to an implementation of BLAS and LAPACK, amongst others.
Bindings are available in [blas-sys][5] and [lapack-sys][6], and wrappers are
available in [blas][7] and [lapack][8].

[1]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
[2]: https://en.wikipedia.org/wiki/LAPACK
[3]: https://www.rust-lang.org/
[4]: https://developer.apple.com/library/mac/documentation/Accelerate/Reference/AccelerateFWRef/

[5]: https://github.com/stainless-steel/blas-sys
[6]: https://github.com/stainless-steel/lapack-sys
[7]: https://github.com/stainless-steel/blas
[8]: https://github.com/stainless-steel/lapack
