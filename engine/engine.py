import cbor2
from pathlib import Path

# Diretórios de teste
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")

# Certifique-se que os diretórios existem
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

def generate_test_opfs():
    """
    Gera automaticamente 5 OPFs de teste:
      - 2 válidos
      - 3 inválidos (extra_field, policy_conflict, assinatura inválida)
    Arquivos são CBOR canônico, compatíveis com o Verifier Rust.
    """

    # --- OPFs válidos ---
    opfs_valid = {
        "opf_valid_minimal.cbor": {
            "name": "minimal",
            "version": 1,
            "policy": "ok",
        },
        "opf_valid_restricted.cbor": {
            "name": "restricted",
            "version": 1,
            "policy": "restricted",
        },
    }

    for filename, data in opfs_valid.items():
        path = VALID_DIR / filename
        with open(path, "wb") as f:
            # CBOR canônico
            f.write(cbor2.dumps(data, canonical=True))
        print(f"[VALID] {path} gerado.")

    # --- OPFs inválidos ---
    opfs_invalid = {
        "opf_invalid_extra_field.cbor": {
            "name": "invalid_extra",
            "version": 1,
            "policy": "ok",
            "extra_field": "evil",  # campo extra inválido
        },
        "opf_invalid_policy_conflict.cbor": {
            "name": "invalid_policy",
            "version": 1,
            "policy": "conflict",  # conflito de política
        },
        "opf_invalid_signature.cbor": {
            "name": "invalid_signature",
            "version": 1,
            "policy": "ok",
            "signature": b"bad",  # assinatura inválida
        },
    }

    for filename, data in opfs_invalid.items():
        path = INVALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(data, canonical=True))
        print(f"[INVALID] {path} gerado.")

if __name__ == "__main__":
    generate_test_opfs()
