import cbor2
import json
from pathlib import Path
import subprocess
import sys
from z3 import Solver, Int, Or, And, sat
from nacl.signing import SigningKey
from datetime import datetime

# ============================
# Diretórios de teste
# ============================
VALID_DIR = Path("tests/valid")
INVALID_DIR = Path("tests/invalid")
REPORT_DIR = Path("reports")
VALID_DIR.mkdir(parents=True, exist_ok=True)
INVALID_DIR.mkdir(parents=True, exist_ok=True)
REPORT_DIR.mkdir(parents=True, exist_ok=True)

# ============================
# IR Enterprise Máxima
# ============================
class IRNode:
    def __init__(self, op_type, value=None, children=None):
        self.op_type = op_type
        self.value = value
        self.children = children or []

    def traverse_paths(self):
        if self.op_type == "loop":
            paths = []
            for val in self.value:
                for child in self.children:
                    for p in child.traverse_paths():
                        paths.append([IRNode("assign", value=val)] + p)
            return paths
        elif self.op_type == "branch":
            paths = []
            for val in self.value:
                for child in self.children:
                    for p in child.traverse_paths():
                        paths.append([IRNode("branch", value=val)] + p)
            return paths
        elif not self.children:
            return [[self]]
        else:
            paths = [[]]
            for child in self.children:
                new_paths = []
                for p in paths:
                    for sp in child.traverse_paths():
                        new_paths.append(p + sp)
                paths = new_paths
            return [[self] + p for p in paths]

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

SIGNING_KEY = SigningKey.generate()

def sign_opf(opf: OPF):
    data = cbor2.dumps(opf.to_dict(), canonical=True)
    return SIGNING_KEY.sign(data).signature

# ============================
# Construção da IR Enterprise
# ============================
def build_ir():
    bar_body = IRNode("loop", value=range(1, 3), children=[
        IRNode("policy_check")
    ])
    foo_body = IRNode("call", value="foo", children=[
        IRNode("assign", value=42),
        IRNode("branch", value=["ok", "restricted"], children=[bar_body])
    ])
    main_body = IRNode("call", value="main", children=[
        IRNode("assign", value=1),
        IRNode("branch", value=["ok", "restricted", "conflict"], children=[IRNode("policy_check")]),
        foo_body
    ])
    return main_body

# ============================
# SMT Solver para paths válidos e inválidos
# ============================
def smt_paths():
    solver = Solver()
    version = Int("version")
    policy = Int("policy")  # 0=ok,1=restricted,2=conflict

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
        solver.add(policy != pol_val)
    return paths

# ============================
# Gerar OPFs e provas formais
# ============================
def generate_opfs_and_proofs():
    paths = smt_paths()
    valid_opfs, invalid_opfs = [], []
    proofs = []

    for idx, path in enumerate(paths, start=1):
        policy = path['policy']
        name = f"opf_{policy}_{idx}"
        opf = OPF(name=name, version=1, policy=policy)
        opf.signature = sign_opf(opf) if policy != "conflict" else None

        # Classificação
        if policy == "conflict":
            invalid_opfs.append(opf)
        else:
            valid_opfs.append(opf)

        # Prova formal detalhada
        proof = {
            "opf_name": name,
            "policy": policy,
            "path_id": idx,
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "valid": policy != "conflict",
            "signature": opf.signature.hex() if opf.signature else None
        }
        proofs.append(proof)

    return valid_opfs, invalid_opfs, proofs

# ============================
# Escrever OPFs
# ============================
def write_opfs(opfs, valid=True):
    for opf in opfs:
        dir_path = VALID_DIR if valid else INVALID_DIR
        path = dir_path / f"{opf.name}.cbor"
        with open(path, "wb") as f:
            f.write(cbor2.dumps(opf.to_dict(), canonical=True))
        print(f"[{'VALID' if valid else 'INVALID'}] {path} gerado.")

# ============================
# Escrever relatório de provas
# ============================
def write_proof_report(proofs):
    report_file = REPORT_DIR / f"opf_proofs_{datetime.utcnow().strftime('%Y%m%dT%H%M%S')}.json"
    with open(report_file, "w") as f:
        json.dump(proofs, f, indent=4)
    print(f"📄 Relatório de provas formais gerado: {report_file}")

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
# Pipeline completo Fase 3
# ============================
def run_pipeline():
    print("=== Fase 3 Enterprise: Engine Formal + Provas Formais + CI/CD ===")
    valid_opfs, invalid_opfs, proofs = generate_opfs_and_proofs()
    write_opfs(valid_opfs, valid=True)
    write_opfs(invalid_opfs, valid=False)
    write_proof_report(proofs)
    run_rust_tests()

if __name__ == "__main__":
    run_pipeline()
