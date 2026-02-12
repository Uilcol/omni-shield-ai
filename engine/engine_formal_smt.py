import cbor2
from pathlib import Path
from hashlib import sha256
import subprocess
import itertools
import sys

# --- Diretórios de teste ---
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

# --- IR interprocedural simplificada ---
class IRNode:
    """Representa um nó na IR: ação ou decisão"""
    def __init__(self, op_type, value=None):
        self.op_type = op_type  # "policy_check", "assign", "call"
        self.value = value      # valor associado

class OPF:
    """OPF gerada a partir da IR e caminhos simbólicos"""
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

# --- Funções auxiliares ---
def sign_opf(opf: OPF):
    """Placeholder de assinatura compatível com Verifier"""
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return sha256(data).digest()[:32]

def symbolic_policy_paths():
    """
    Simula caminhos SMT simbólicos:
    - policy_ok
    - policy_restricted
    - policy_conflict
    """
    return ["ok", "restricted", "conflict"]

# --- Geração formal de OPFs válidos ---
def generate_valid_opfs():
    print("Gerando OPFs válidos formais (todos os caminhos SMT)...")
    valid_policies = ["ok", "restricted"]
    opfs = []

    for idx, policy in enumerate(valid_policies, start=1):
        opf = OPF(name=f"formal_valid_{policy}", version=1, policy=policy)
        opf.signature = sign_opf(opf)
        filename = f"opf_valid_{policy}.cbor"
        path = VALID_DIR / filename
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[VALID-FORMAL] {path} gerado.")
        opfs.append(opf)
    return opfs

# --- Geração formal de OPFs inválidos ---
def generate_invalid_opfs():
    print("Gerando OPFs inválidos formais (caminhos impossíveis)...")
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

# --- Executa testes Rust e captura resultados ---
def run_rust_tests():
    print("\n🔹 Executando testes Rust...")
    try:
        result = subprocess.run(
            ["cargo", "test", "--tests", "-p", "omniuil_verifier", "--", "--nocapture"],
            cwd=".",  # raiz do projeto
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

        print("\n📊 Resumo por OPF/teste:")
        for name, status in summary.items():
            print(f"- {name}: {status}")

        if all(s == "ok" for s in summary.values()):
            print("\n✅ Todos os testes passaram com OPFs formais gerados.")
        else:
            print("\n⚠️ Alguns testes falharam. Verifique os logs acima.")

    except FileNotFoundError:
        print("❌ Cargo/Rust ou Python não encontrados. Verifique a instalação e o PATH.")
        sys.exit(1)

# --- Pipeline completo Fase 2 ---
def run_pipeline():
    print("=== Fase 2 Avançada: Engine Formal IR + SMT ===")
    generate_valid_opfs()
    generate_invalid_opfs()
    run_rust_tests()

if __name__ == "__main__":
    run_pipeline()
