import cbor2
import copy
import os

BASE_DIR = os.path.dirname(__file__)
VALID_DIR = os.path.abspath(os.path.join(BASE_DIR, "..", "test_vectors", "valid"))
INVALID_DIR = os.path.abspath(os.path.join(BASE_DIR, "..", "test_vectors", "invalid"))

os.makedirs(INVALID_DIR, exist_ok=True)

def load_opf(name):
    path = os.path.join(VALID_DIR, name)
    with open(path, "rb") as f:
        return cbor2.load(f)

def save_opf(name, opf):
    path = os.path.join(INVALID_DIR, name)
    with open(path, "wb") as f:
        f.write(cbor2.dumps(opf, canonical=True))

# -------------------------------------------------
# 1) OPF INVÁLIDA — campo extra (violação estrutural)
# -------------------------------------------------
def invalid_extra_field():
    opf = load_opf("opf_valid_minimal.cbor")
    opf["evil"] = True  # CAMPO PROIBIDO
    save_opf("opf_invalid_extra_field.cbor", opf)

# -------------------------------------------------
# 2) OPF INVÁLIDA — conflito de política
# scope=process + filesystem.exec
# -------------------------------------------------
def invalid_policy_conflict():
    opf = load_opf("opf_valid_minimal.cbor")

    opf["policy"]["execution"]["scope"] = "process"
    opf["policy"]["filesystem"] = {
        "read": [],
        "write": [],
        "exec": ["/bin/sh"]  # CONFLITO SEMÂNTICO
    }

    save_opf("opf_invalid_policy_conflict.cbor", opf)

# -------------------------------------------------
# 3) OPF INVÁLIDA — assinatura corrompida
# -------------------------------------------------
def invalid_signature():
    opf = load_opf("opf_valid_minimal.cbor")

    sig = bytearray(opf["signature"]["sig"])
    sig[0] ^= 0xFF  # flip de 1 byte
    opf["signature"]["sig"] = bytes(sig)

    save_opf("opf_invalid_signature.cbor", opf)

# -------------------------------------------------

if __name__ == "__main__":
    invalid_extra_field()
    invalid_policy_conflict()
    invalid_signature()

    print("OPFs inválidas geradas com sucesso.")
