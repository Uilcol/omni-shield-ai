import cbor2
from pathlib import Path
from hashlib import sha256
import subprocess
import sys
from z3 import Solver, Int, Or, And, sat

# ============================
# Diretórios de teste
# ============================
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)

# ============================
# IR Interprocedural avançada
# ============================
class IRNode:
    """Nó da IR: tipo, valor simbólico, filhos"""
    def __init__(self, op_type, value=None, children=None):
        self.op_type = op_type  # assign, policy_check, call, branch
        self.value = value
        self.children = children or []

# ============================
# OPF
# ============================
class OPF:
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
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return sha256(data).digest()[:32]

# ============================
# Construção da IR
# ============================
def build_ir():
    """
    IR com:
    - assign version
    - branch policy_check
    - chamada de função foo
    """
    assign_version = IRNode("assign", value=1)

    # Branch simbólica de política
    branch_policy = IRNode("branch", value=["ok", "restricted", "conflict"],
                           children=[IRNode("policy_check")])

    # Função foo chamada pelo main
    foo_body = IRNode("call", value="foo", children=[IRNode("assign", value=42),
                                                     IRNode("policy_check")])

    call_main = IRNode("call", value="main", children=[assign_version, branch_policy, foo_body])
    return call_main

# ============================
# SMT – Eliminação formal de branches impossíveis
# ============================
def smt_paths():
    solver = Solver()
    version = Int('version')
    policy = Int('policy')  # 0=ok,1=restricted,2=conflict

    # Restrição exemplo: conflict só é inválido, ok e restricted válidos
    solver.add(Or(policy == 0, policy == 1, policy == 2))
    solver.add(Or(version > 0, version == 1))  # sempre satisfatório
    # Adicionar mais constraints se necessário

    # Resolver caminhos
    paths = []
    while solver.check() == sat:
        model = solver.model()
        pol_val = model[policy].as_long()
        path = {}
        if pol_val == 0:
            path['policy'] = 'ok'
        elif pol_val == 1:
            path['policy'] = 'restricted'
        else:
            path['policy'] = 'conflict'
        paths.append(path)
        # Bloquear este modelo para buscar outros
        solver.add(Or(policy != pol_val))
    return paths

# ============================
# Gerar OPFs válidos e inválidos
# ============================
def generate_opfs_smt():
    paths = smt_paths()
    valid_opfs = []
    invalid_opfs = []

    for idx, path in enumerate(paths, start=1):
        policy = path['policy']
        name = f"opf_{policy}_{idx}"
        opf = OPF(name=name, version=1, policy=policy)

        if policy == "conflict":
            invalid_opfs.append(opf)
        else:
            opf.signature = sign_opf(opf)
            valid_opfs.append(opf)
    return valid_opfs, invalid_opfs

# ============================
# Escrever OPFs
# ============================
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
    except FileNotFoundError:
        print("❌ Cargo/Rust ou Python não encontrados. Verifique PATH.")
        sys.exit(1)

# ============================
# Pipeline completo Fase 2 – SMT real
# ============================
def run_pipeline():
    print("=== Fase 2 Avançada: Engine Formal IR + SMT real ===")
    valid_opfs, invalid_opfs = generate_opfs_smt()
    write_opfs(valid_opfs, valid=True)
    write_opfs(invalid_opfs, valid=False)
    run_rust_tests()

if __name__ == "__main__":
    run_pipeline()
