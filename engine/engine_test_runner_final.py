import cbor2
from pathlib import Path
from hashlib import sha256
import subprocess
import sys

# ============================
# Diretórios de teste
# ============================
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

# ============================
# IR + OPF
# ============================
class IRNode:
    """Nó de IR simplificado: policy_check, assign, call"""
    def __init__(self, op_type, value=None):
        self.op_type = op_type
        self.value = value

class OPF:
    """Representa um OPF"""
    def __init__(self, name, version, policy, extra_field=None, signature=None):
        self.name = name
        self.version = version
        self.policy = policy
        self.extra_field = extra_field
        self.signature = signature

    def to_dict(self):
        d = {"name": self.name, "version": self.version, "policy": self.policy}
        if self.extra_field is not None:
            d["extra_field"] = self.extra_field
        if self.signature is not None:
            d["signature"] = self.signature
        return d

# ============================
# Placeholder assinatura
# ============================
def sign_opf(opf: OPF):
    """Simula assinatura Ed25519 compatível"""
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return sha256(data).digest()[:32]

# ============================
# Caminhos SMT simbólicos
# ============================
def symbolic_policies():
    """Todos os caminhos possíveis de política"""
    return ["ok", "restricted", "conflict"]

# ============================
# Gerar OPFs válidos
# ============================
def generate_valid_opfs():
    print("🔹 Gerando OPFs válidos formais...")
    valid_policies = ["ok", "restricted"]
    opfs = []

    for policy in valid_policies:
        opf = OPF(name=f"formal_valid_{policy}", version=1, policy=policy)
        opf.signature = sign_opf(opf)
        filename = f"opf_valid_{policy}.cbor"
        path = VALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[VALID-FORMAL] {path} gerado.")
        opfs.append(opf)
    return opfs

# ============================
# Gerar OPFs inválidos
# ============================
def generate_invalid_opfs():
    print("🔹 Gerando OPFs inválidos formais...")
    invalid_combinations = [
        {"extra_field": "evil", "policy": "ok"},
        {"extra_field": None, "policy": "conflict"},
        {"extra_field": None, "policy": "ok", "signature": b"bad"},
    ]
    opfs = []

    for combo in invalid_combinations:
        opf = OPF(
            name=f"formal_invalid_{combo.get('policy','unknown')}",
            version=1,
            policy=combo.get("policy", "unknown"),
            extra_field=combo.get("extra_field"),
            signature=combo.get("signature") or sign_opf(OPF("temp",1,combo.get("policy")))
        )
        filename = f"opf_invalid_{opf.policy}.cbor"
        path = INVALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[INVALID-FORMAL] {path} gerado.")
        opfs.append(opf)
    return opfs

# ============================
# Executar testes Rust
# ============================
def run_rust_tests():
    print("\n🔹 Executando testes Rust...")
    try:
        result = subprocess.run(
            ["cargo", "test", "--tests", "-p", "omniuil_verifier", "--", "--nocapture"],
            cwd=".",
            capture_output=True,
            text=True,
            check=False
        )

        print(result.stdout)
        print(result.stderr)

        summary = {}
        for line in result.stdout.splitlines():
            if line.startswith("test"):
                parts = line.split("...")
                if len(parts) == 2:
                    test_name = parts[0].replace("test", "").strip()
                    status = parts[1].strip()
                    summary[test_name] = status

        print("\n📊 Resumo final por OPF/teste:")
        for name, status in summary.items():
            print(f"- {name}: {status}")

        if all(s == "ok" for s in summary.values()):
            print("\n✅ Todos os testes passaram com OPFs formais.")
        else:
            print("\n⚠️ Alguns testes falharam. Verifique os logs acima.")

    except FileNotFoundError:
        print("❌ Cargo/Rust ou Python não encontrados. Verifique PATH.")
        sys.exit(1)

# ============================
# Pipeline completo Fase 2
# ============================
def run_pipeline():
    print("=== Fase 2: Engine Formal IR + SMT — Pipeline completo ===")
    generate_valid_opfs()
    generate_invalid_opfs()
    run_rust_tests()

# ============================
if __name__ == "__main__":
    run_pipeline()
