# trs.js
[![GitHub issues](https://img.shields.io/github/issues/VCNinc/trs.js)](https://github.com/VCNinc/trs.js/issues)
[![GitHub license](https://img.shields.io/github/license/VCNinc/trs.js)](https://github.com/VCNinc/trs.js/blob/master/LICENSE)
[![NPM version](https://img.shields.io/npm/v/trs-js)](https://www.npmjs.com/package/trs-js)

A port of [rozbb/fujisaki-ringsig](https://github.com/rozbb/fujisaki-ringsig) to WebAssembly for use in browsers and Node.js.

Contains an implementation of the of the [Traceable Ring Signature algorithm by Eiichiro Fujisaki and Koutarou Suzuki](https://eprint.iacr.org/2006/389.pdf).

## Usage

Install the package via NPM:

```
npm i --save trs-js
```

Require the package in your code:
```
const trs = require('./pkg/trs_js.js')
```

Call the functions like so:
```
trs.gen_keypair()
trs.public_to_ascii(public_key)
trs.ascii_to_public(ascii)
trs.generate_signature(message, pki, issue, private_key)
trs.signature_to_ascii(signature)
trs.ascii_to_signature(ascii)
trs.verify_signature(message, pki, issue, signature)
trs.trace_signature(message_1, signature_1, message_2, signature_2, pki, issue)
```

Check out [example.js](example.js) for a full example.

## Authors

The original rust code is by Michael Rosenberg: [rozbb](https://github.com/rozbb).
Ported to WebAssembly by Vivek Nair: [VCNinc](https://github.com/VCNinc).

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT)).

## Warning

As with the original Rust crate, this package should not be used in production code. It contains experimental cryptography and uses small (insecure) keys.
