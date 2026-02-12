import cbor2, hashlib


def emit_opf(ir_bytes, policy, signature):
ir_hash = hashlib.sha256(ir_bytes).digest()
opf = {
'version': 1,
'ir_hash': ir_hash,
'policy': policy,
'signature': signature,
}
return cbor2.dumps(opf, canonical=True)