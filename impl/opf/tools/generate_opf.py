import os
import cbor2
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives import serialization
from hashlib import sha256
from datetime import datetime

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
VALID_DIR = os.path.abspath(os.path.join(BASE_DIR, "..", "test_vectors", "valid"))

# GARANTE que o diretório exista
os.makedirs(VALID_DIR, exist_ok=True)

def generate_keypair():
    priv = Ed25519PrivateKey.generate()
    pub = priv.public_key()
    return priv, pub

def sign(priv, data: bytes) -> bytes:
    return priv.sign(data)

def base_opf(ir_hash: bytes):
    return {
        "version": 1,
        "metadata": {
            "created_at": datetime.utcnow().isoformat() + "Z",
            "generator": "omniuil-engine-test",
            "target": "unit-test"
        },
        "ir_hash": ir_hash,
        "smt": {
            "solver": "z3",
            "result": "sat",
            "constraints": [b"(assert true)"]
        },
        "policy": {
            "execution": {
                "mode": "allow",
                "scope": "process"
            }
        },
        "hardware": None
    }

def emit_opf(filename, opf, priv, pub):
    unsigned = dict(opf)
    unsigned.pop("signature", None)

    canonical = cbor2.dumps(unsigned, canonical=True)
    sig = sign(priv, canonical)

    opf["signature"] = {
        "alg": "ed25519",
        "pubkey": pub.public_bytes(
            encoding=serialization.Encoding.Raw,
            format=serialization.PublicFormat.Raw
        ),
        "sig": sig
    }

    path = os.path.join(VALID_DIR, filename)
    with open(path, "wb") as f:
        f.write(cbor2.dumps(opf, canonical=True))

if __name__ == "__main__":
    ir_hash = sha256(b"sample-ir-deterministic").digest()
    priv, pub = generate_keypair()

    emit_opf("opf_valid_minimal.cbor", base_opf(ir_hash), priv, pub)

    restricted = base_opf(ir_hash)
    restricted["policy"]["filesystem"] = {
        "read": ["/usr/lib"],
        "write": [],
        "exec": []
    }
    emit_opf("opf_valid_restricted.cbor", restricted, priv, pub)

    print("OPFs válidas geradas com sucesso.")
