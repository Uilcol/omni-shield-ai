# engine/opf_emitter.py
import hashlib
import json

def generate_opf(ir_hash: str, extra_field=False, policy_conflict=False, signature_valid=True) -> bytes:
    """
    Gera OPF determinístico compatível com Verifier Rust
    """
    opf = {
        "ir_hash": ir_hash,
        "has_extra_field": extra_field,
        "policy_conflict": policy_conflict,
        "signature_valid": signature_valid
    }
    # Serialização determinística CBOR fake (usamos JSON ordenado para placeholder)
    opf_bytes = json.dumps(opf, sort_keys=True).encode("utf-8")
    return opf_bytes

def hash_ir(ir_data: str) -> str:
    """
    Hash determinístico do IR (SHA256)
    """
    h = hashlib.sha256()
    h.update(ir_data.encode("utf-8"))
    return h.hexdigest()
