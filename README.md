# accelerate-src [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides [BLAS][1] and [LAPACK][2] using Apple’s [Accelerate
framework][3]. The framework is shipped with Mac OS X, and the package only
links to it (without any additional compilation). Note that any other functions
exported by the framework are implicitly made available by the package.

## Where are all the FFI definitions?

This package provides only an implementation of BLAS and LAPACK. Bindings are
available in [blas-sys][4] and [lapack-sys][5], and wrappers are available in
[blas][6] and [lapack][7].

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
[2]: https://en.wikipedia.org/wiki/LAPACK
[3]: https://developer.apple.com/library/mac/documentation/Accelerate/Reference/AccelerateFWRef

[4]: https://github.com/stainless-steel/blas-sys
[5]: https://github.com/stainless-steel/lapack-sys
[6]: https://github.com/stainless-steel/blas
[7]: https://github.com/stainless-steel/lapack

[status-img]: https://travis-ci.org/strawlab/accelerate-src.svg?branch=master
[status-url]: https://travis-ci.org/strawlab/accelerate-src
[version-img]: https://img.shields.io/crates/v/accelerate-src.svg
[version-url]: https://crates.io/crates/accelerate-src
