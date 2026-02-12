import cbor2
from pathlib import Path
from hashlib import sha256

# Diretórios de saída de OPFs
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

# --- IR simplificada para OPFs ---
class OPF:
    def __init__(self, name, version, policy, extra_field=None, signature=None):
        self.name = name
        self.version = version
        self.policy = policy
        self.extra_field = extra_field
        self.signature = signature

    def to_dict(self):
        d = {
            "name": self.name,
            "version": self.version,
            "policy": self.policy
        }
        if self.extra_field is not None:
            d["extra_field"] = self.extra_field
        if self.signature is not None:
            d["signature"] = self.signature
        return d

# --- Funções auxiliares ---
def sign_opf(opf: OPF):
    """
    Gera placeholder de assinatura Ed25519 compatível com Verifier.
    Aqui usamos hash simples como placeholder.
    """
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return sha256(data).digest()[:32]  # 32 bytes simulando assinatura

# --- Gerar OPFs válidos formalmente ---
def generate_valid_opfs():
    opfs = []

    # OPF minimal
    minimal = OPF(name="minimal", version=1, policy="ok")
    minimal.signature = sign_opf(minimal)
    opfs.append(("opf_valid_minimal.cbor", minimal))

    # OPF restricted
    restricted = OPF(name="restricted", version=1, policy="restricted")
    restricted.signature = sign_opf(restricted)
    opfs.append(("opf_valid_restricted.cbor", restricted))

    for filename, opf in opfs:
        path = VALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[VALID-FORMAL] {path} gerado.")

# --- Gerar OPFs inválidos formalmente ---
def generate_invalid_opfs():
    opfs = []

    # Extra field inválido
    extra = OPF(name="invalid_extra", version=1, policy="ok", extra_field="evil")
    extra.signature = sign_opf(extra)
    opfs.append(("opf_invalid_extra_field.cbor", extra))

    # Policy conflict
    conflict = OPF(name="invalid_policy", version=1, policy="conflict")
    conflict.signature = sign_opf(conflict)
    opfs.append(("opf_invalid_policy_conflict.cbor", conflict))

    # Assinatura inválida
    bad_sig = OPF(name="invalid_signature", version=1, policy="ok")
    bad_sig.signature = b"bad"
    opfs.append(("opf_invalid_signature.cbor", bad_sig))

    for filename, opf in opfs:
        path = INVALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[INVALID-FORMAL] {path} gerado.")

# --- Pipeline completo ---
def run_pipeline():
    generate_valid_opfs()
    generate_invalid_opfs()

if __name__ == "__main__":
    run_pipeline()
