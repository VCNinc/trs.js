# trs.js

A port of [rozbb/fujisaki-ringsig](https://github.com/rozbb/fujisaki-ringsig) to WebAssembly for use in browsers and Node.js.

Contains an implementation of the of the [Traceable Ring Signature algorithm by Eiichiro Fujisaki and Koutarou Suzuki](https://eprint.iacr.org/2006/389.pdf).

## Authors

The original rust code is by Michael Rosenberg: [rozbb](https://github.com/rozbb).
Ported to WebAssembly by Vivek Nair: [VCNinc](https://github.com/VCNinc).

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT)).

## Warning

As with the original Rust crate, this package should not be used in production code. It contains experimental cryptography and uses small (insecure) keys.
