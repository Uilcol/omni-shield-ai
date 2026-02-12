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
# IR Interprocedural Profissional
# ============================
class IRNode:
    """
    Nó da IR:
    - op_type: assign, policy_check, call, branch
    - value: valor simbólico (int, str)
    - children: lista de nós filhos
    """
    def __init__(self, op_type, value=None, children=None):
        self.op_type = op_type
        self.value = value
        self.children = children or []

    def traverse_paths(self):
        """
        Retorna todos os caminhos possíveis da IR
        Cada branch gera múltiplos caminhos (SMT simbólico)
        """
        if not self.children:
            return [[self]]

        paths = []
        for child in self.children:
            sub_paths = child.traverse_paths()
            for sp in sub_paths:
                paths.append([self] + sp)
        return paths

# ============================
# OPF Enterprise
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
# Função de assinatura determinística
# ============================
def sign_opf(opf: OPF):
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return sha256(data).digest()[:32]

# ============================
# Construção da IR Interprocedural
# ============================
def build_ir_enterprise():
    """
    IR profissional com:
    - funções chamadas entre si
    - branches simbólicos
    - policy checks
    """
    # Função foo
    foo_body = IRNode("call", value="foo", children=[
        IRNode("assign", value=42),
        IRNode("branch", value=["ok", "restricted"], children=[IRNode("policy_check")])
    ])

    # Função main
    main_body = IRNode("call", value="main", children=[
        IRNode("assign", value=1),
        IRNode("branch", value=["ok", "restricted", "conflict"], children=[IRNode("policy_check")]),
        foo_body
    ])
    return main_body

# ============================
# SMT Solver Enterprise
# ============================
def smt_paths_enterprise():
    solver = Solver()
    version = Int("version")
    policy = Int("policy")  # 0=ok, 1=restricted, 2=conflict

    # Regras de consistência
    solver.add(Or(policy == 0, policy == 1, policy == 2))
    solver.add(version > 0)

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
        solver.add(policy != pol_val)  # Bloquear modelo repetido
    return paths

# ============================
# Gerar OPFs Enterprise
# ============================
def generate_opfs_enterprise():
    paths = smt_paths_enterprise()
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
        print(f"[{'VALID' if valid else 'INVALID'}] {path} gerado.")

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
# Pipeline completo – Fase 2 Enterprise
# ============================
def run_pipeline():
    print("=== Fase 2 Enterprise: Engine Formal IR Interprocedural + SMT real ===")
    valid_opfs, invalid_opfs = generate_opfs_enterprise()
    write_opfs(valid_opfs, valid=True)
    write_opfs(invalid_opfs, valid=False)
    run_rust_tests()

if __name__ == "__main__":
    run_pipeline()
