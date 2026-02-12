import cbor2
from pathlib import Path
from hashlib import sha256
import subprocess
import sys
from itertools import product

# ============================
# Diretórios de teste
# ============================
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

# ============================
# IR Interprocedural
# ============================
class IRNode:
    """Nó de IR: tipo de operação, valor e filhos"""
    def __init__(self, op_type, value=None, children=None):
        self.op_type = op_type  # "assign", "policy_check", "call"
        self.value = value
        self.children = children or []

    def traverse_paths(self):
        """
        Retorna todos os caminhos possíveis a partir deste nó (para SMT simbólico)
        """
        if not self.children:
            return [[self]]
        paths = []
        for child in self.children:
            for sub_path in child.traverse_paths():
                paths.append([self] + sub_path)
        return paths

class OPF:
    """OPF gerada a partir da IR e caminhos SMT"""
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
    """Assinatura simulada Ed25519 compatível com Verifier"""
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return sha256(data).digest()[:32]

# ============================
# Engine Formal
# ============================
def build_ir():
    """
    Cria IR interprocedural simbólica:
    - assign
    - policy_check
    - call (simula função)
    """
    # Função check_policy
    check_policy_node = IRNode("policy_check", value=["ok", "restricted", "conflict"])

    # Função assign_version
    assign_version = IRNode("assign", value=1)

    # Raiz: chamada de função -> assign -> policy_check
    root = IRNode("call", value="main", children=[assign_version, check_policy_node])
    return root

def generate_opfs_from_ir():
    """
    Gera OPFs válidos e inválidos para todos os caminhos SMT possíveis
    """
    root = build_ir()
    paths = root.traverse_paths()
    valid_opfs = []
    invalid_opfs = []

    for idx, path in enumerate(paths, start=1):
        policy_node = next((n for n in path if n.op_type == "policy_check"), None)
        if policy_node is None:
            continue
        for policy in policy_node.value:
            # Deterministic naming
            name = f"opf_{policy}_{idx}"
            opf = OPF(name=name, version=1, policy=policy)

            # Invalid cases
            if policy == "conflict":
                invalid_opfs.append(opf)
            else:
                opf.signature = sign_opf(opf)
                valid_opfs.append(opf)

    return valid_opfs, invalid_opfs

def write_opfs(opfs, valid=True):
    for opf in opfs:
        dir_path = VALID_DIR if valid else INVALID_DIR
        filename = f"{opf.name}.cbor"
        path = dir_path / filename
        if opf.signature is None and valid:
            opf.signature = sign_opf(opf)
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[{'VALID-FORMAL' if valid else 'INVALID-FORMAL'}] {path} gerado.")

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
            print("\n✅ Todos os testes passaram com OPFs formais interprocedurais.")
        else:
            print("\n⚠️ Alguns testes falharam. Verifique os logs acima.")

    except FileNotFoundError:
        print("❌ Cargo/Rust ou Python não encontrados. Verifique PATH.")
        sys.exit(1)

# ============================
# Pipeline completo Fase 2 avançada
# ============================
def run_pipeline():
    print("=== Fase 2 Avançada: Engine Formal IR Interprocedural + SMT ===")
    valid_opfs, invalid_opfs = generate_opfs_from_ir()
    write_opfs(valid_opfs, valid=True)
    write_opfs(invalid_opfs, valid=False)
    run_rust_tests()

# ============================
if __name__ == "__main__":
    run_pipeline()
