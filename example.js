const trs = require('./pkg/trs_js.js')

console.log('generating key pairs...')
const kp1 = trs.gen_keypair()
const pk1 = kp1.public
const sk1 = kp1.private
const kp2 = trs.gen_keypair()
const pk2 = kp2.public
const sk2 = kp2.private

console.log('generating ASCII keys...')
const ak1 = trs.public_to_ascii(pk1)
const ak2 = trs.public_to_ascii(pk2)
const pki = [ak1, ak2]
console.log(pki)

console.log('generating signatures...')
const issue = "test";
const sig1 = trs.generate_signature("msg1", pki, issue, sk1)
const sig2 = trs.generate_signature("msg2", pki, issue, sk2)
const sig3 = trs.generate_signature("msg3", pki, issue, sk2)

console.log('generating ASCII signatures...')
const as1 = trs.signature_to_ascii(sig1)
const as2 = trs.signature_to_ascii(sig2)
const as3 = trs.signature_to_ascii(sig3)
console.log([as1, as2, as3])

console.log('verifying signatures from ASCII...')
const s1 = trs.ascii_to_signature(as1)
const s2 = trs.ascii_to_signature(as2)
const s3 = trs.ascii_to_signature(as3)
const v1 = trs.verify_signature("msg1", pki, issue, s1)
const v2 = trs.verify_signature("msg2", pki, issue, s2)
const v3 = trs.verify_signature("msg3", pki, issue, s3)
console.log([v1, v2, v3])

console.log('these invalid signatures should not verify...')
const f1 = trs.verify_signature("another1", pki, issue, s1)
const f2 = trs.verify_signature("another2", pki, issue, s2)
const f3 = trs.verify_signature("another3", pki, issue, s3)
console.log([f1, f2, f3])

console.log('tracing independent signatures...')
const t1 = trs.trace_signature("msg1", s1, "msg2", s2, pki, issue);
console.log([t1]);

console.log('tracing revealed signatures...')
const t2 = trs.trace_signature("msg2", s2, "msg3", s3, pki, issue);
console.log([t2, ak2]);
